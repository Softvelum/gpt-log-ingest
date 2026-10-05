#!/usr/bin/env python3
"""Python 3 fallback for the grouped Nimble runtime log watcher."""

from __future__ import annotations

import argparse
import heapq
import json
import re
import sys
import time
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MAP = ROOT / "logger/search-index.json"
LEVELS = {"E": "error", "V": "verbose", "D": "debug", "I": "info"}
TOKEN_RE = re.compile(r"[A-Za-z0-9]+")
MAX_BYTES_PER_FILE_PER_POLL = 4 * 1024 * 1024
MAX_LAG_BYTES = 32 * 1024
NIMBLE_RECORD_PREFIX = re.compile(rb"^\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} ")
ANSI = {"STATE": "\x1b[36m", "BATCH": "\x1b[34m", "PERF": "\x1b[35m", "GAP": "\x1b[33m", "RECORD": "\x1b[31m", "ERROR": "\x1b[31m"}


def emit(tag: str, message: str, color: bool) -> None:
    prefix = f"[WATCH][{tag}]"
    if color:
        print(f"{ANSI.get(tag, '')}{prefix}\x1b[0m {message}", flush=True)
    else:
        print(f"{prefix} {message}", flush=True)


def runtime_logs(directory: Path) -> list[Path]:
    return sorted(
        path
        for path in directory.iterdir()
        if path.is_file()
        and (path.name == "nimble.log" or (path.name.startswith("nimble-") and path.name.endswith(".log")))
    )


def identity(path: Path) -> str:
    stat = path.stat()
    return f"{stat.st_dev}:{stat.st_ino}" if stat.st_ino else str(path.resolve())


def resync_to_record(path: Path, offset: int, size: int) -> int:
    """Find the first complete-line boundary with a Nimble timestamp prefix."""
    start = max(offset, size - MAX_LAG_BYTES)
    with path.open("rb") as stream:
        if start and start > offset:
            stream.seek(start - 1)
            at_line_start = stream.read(1) == b"\n"
        else:
            at_line_start = True
        stream.seek(start)
        data = stream.read(max(0, size - start))
    cursor = 0
    if not at_line_start:
        newline = data.find(b"\n")
        if newline < 0:
            return size
        cursor = newline + 1
    while cursor < len(data):
        newline = data.find(b"\n", cursor)
        line_end = len(data) if newline < 0 else newline
        if NIMBLE_RECORD_PREFIX.match(data[cursor:line_end]):
            return start + cursor
        if newline < 0:
            break
        cursor = newline + 1
    return size


def parse_record(line: str) -> tuple[str, str, str, str] | None:
    first_close = line.find("] [")
    if not line.startswith("[") or first_close < 0:
        return None
    timestamp = " ".join(line[1:first_close].split()[:2])
    component_start = first_close + 3
    component_end = line.find("] ", component_start)
    if component_end < 0:
        return None
    component = line[component_start:component_end]
    message = line[component_end + 2 :]
    if len(message) < 2 or message[1] != ":" or message[0] not in LEVELS:
        return None
    body = message[2:]
    if body.startswith(" "):
        body = body[1:]
    return message[0], component, timestamp, body


def candidates(index: dict, body: str, level_code: str, limit: int = 3) -> list[dict]:
    scores: Counter[int] = Counter()
    for token in TOKEN_RE.findall(body):
        for entry_index in index["terms"].get(token.lower(), []):
            scores[entry_index] += 1
    level = LEVELS[level_code]
    def ranked_rows():
        for entry_index, score in scores.items():
            entry = index["entries"][entry_index]
            agrees = entry["level"] == level or (level == "info" and entry["level"] == "info (always)")
            yield (not agrees, -score, entry["id"], score, agrees, entry)

    rows = heapq.nsmallest(limit, ranked_rows(), key=lambda row: row[:3])
    return [
        {"id": row[5]["id"], "title": row[5]["title"], "score": row[3], "level_agrees": row[4]}
        for row in rows[:limit]
    ]


def make_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dir", required=True, type=Path, help="Nimble log directory")
    parser.add_argument("--map", type=Path, default=DEFAULT_MAP, help="generated logger/search-index.json")
    parser.add_argument("--levels", default="E", help="comma-separated exact level codes: E,V,D,I")
    parser.add_argument("--interval-seconds", type=int, default=10)
    parser.add_argument("--duration-seconds", type=int, help="omit to watch until interrupted")
    parser.add_argument("--contains", help="include only messages whose body contains this text")
    parser.add_argument("--include-existing", action="store_true", help="include current log contents")
    parser.add_argument("--color", choices=("auto", "always", "never"), default="auto")
    parser.add_argument("--metrics-only", action="store_true", help="suppress message bodies and candidate IDs")
    return parser


def run() -> int:
    args = make_parser().parse_args()
    selected = {code.strip() for code in args.levels.split(",") if code.strip()}
    unknown = selected - set(LEVELS)
    if unknown:
        raise ValueError(f"unsupported level code(s): {','.join(sorted(unknown))}; use E, V, D, I")
    if not selected or args.interval_seconds <= 0 or (args.duration_seconds is not None and args.duration_seconds <= 0):
        raise ValueError("select at least one level and use positive interval/duration values")
    if not args.dir.is_dir():
        raise ValueError(f"log directory is not readable: {args.dir}")
    index = json.loads(args.map.read_text(encoding="utf-8"))
    if index.get("schema_version") != 1 or len(index.get("entries", [])) != index.get("entry_count"):
        raise ValueError("logger map is unsupported or inconsistent")
    color = args.color == "always" or (args.color == "auto" and sys.stdout.isatty())

    states = {}
    for path in runtime_logs(args.dir):
        key = identity(path)
        size = path.stat().st_size
        states[key] = {"path": path, "offset": 0 if args.include_existing else size, "size": size}
    emit("STATE", f"status=starting observed={datetime.now(timezone.utc).isoformat()} directory={args.dir} "
         f"files={len(states)} levels={','.join(sorted(selected))} backend=polling "
         f"interval_seconds={args.interval_seconds} backlog={'included' if args.include_existing else 'excluded'}", color)
    started = time.monotonic()
    emit("STATE", f"status=waiting_for_next_poll observed={datetime.now(timezone.utc).isoformat()} interval_seconds={args.interval_seconds}", color)
    try:
        while True:
            if args.duration_seconds is not None and time.monotonic() - started >= args.duration_seconds:
                break
            remaining = None if args.duration_seconds is None else args.duration_seconds - (time.monotonic() - started)
            sleep_seconds = args.interval_seconds if remaining is None else min(args.interval_seconds, max(0, remaining))
            time.sleep(sleep_seconds)
            batch_started = time.perf_counter()
            paths = runtime_logs(args.dir)
            emit("STATE", f"status=parsing observed={datetime.now(timezone.utc).isoformat()} files_pending={len(paths)}", color)
            current = {}
            groups = {}
            gaps = []
            complete_lines = 0
            unclassified = 0
            selected_count = 0
            bytes_read = 0
            read_seconds = 0.0
            parse_seconds = 0.0
            group_seconds = 0.0
            catalog_seconds = 0.0
            for path in paths:
                try:
                    key = identity(path)
                    size = path.stat().st_size
                    state = states.get(key, {"path": path, "offset": 0, "size": 0})
                    offset = state["offset"]
                    if size < offset:
                        gaps.append(f"truncated {path}: offset {offset} > size {size}; resumed at start")
                        offset = 0
                    lag = size - offset
                    if lag > MAX_LAG_BYTES:
                        resumed_at = resync_to_record(path, offset, size)
                        gaps.append(
                            f"lag_bytes={lag} path={path} skipped_range={offset}:{resumed_at} "
                            f"skipped_bytes={resumed_at - offset} resumed_at={resumed_at}"
                        )
                        offset = resumed_at
                    limit = min(max(0, size - offset), MAX_BYTES_PER_FILE_PER_POLL)
                    read_started = time.perf_counter()
                    with path.open("rb") as stream:
                        stream.seek(offset)
                        data = stream.read(limit)
                    read_seconds += time.perf_counter() - read_started
                    bytes_read += len(data)
                    records = data.split(b"\n")
                    completed = records[:-1] if len(records) > 1 else []
                    tail = records[-1]
                    consumed = len(data) - len(tail)
                    byte_offset = offset
                    for raw in completed:
                        line_start = byte_offset
                        byte_offset += len(raw) + 1
                        complete_lines += 1
                        line = raw.decode("utf-8", "replace")
                        parse_started = time.perf_counter()
                        parsed = parse_record(line[:-1] if line.endswith("\r") else line)
                        parse_seconds += time.perf_counter() - parse_started
                        if parsed is None:
                            unclassified += 1
                            continue
                        code, component, timestamp, body = parsed
                        if code not in selected or (args.contains and args.contains not in body):
                            continue
                        selected_count += 1
                        key_group = (code, component, body)
                        record = {"path": str(path), "offset": line_start, "timestamp": timestamp}
                        group_started = time.perf_counter()
                        if key_group not in groups:
                            groups[key_group] = {"count": 1, "first": record, "last": record}
                        else:
                            groups[key_group]["count"] += 1
                            groups[key_group]["last"] = record
                        group_seconds += time.perf_counter() - group_started
                    current[key] = {"path": path, "offset": offset + consumed, "size": size}
                except OSError as error:
                    gaps.append(f"cannot read {path}: {error}")
            missing_keys = []
            for key, state in states.items():
                if key not in current and state["size"]:
                    gaps.append(f"previously watched file identity unavailable: {state['path']} at {state['offset']}")
                    missing_keys.append(key)
            for key in missing_keys:
                states.pop(key, None)
            states.update(current)
            if groups or gaps or args.metrics_only:
                emit("BATCH", f"observed={datetime.now(timezone.utc).isoformat()} files={len(current)} "
                     f"complete_lines={complete_lines} selected_records={selected_count} groups={len(groups)} "
                     f"unclassified={unclassified} gaps={len(gaps)}", color)
                for gap in gaps:
                    emit("GAP", repr(gap), color)
                output_started = time.perf_counter()
                for (code, component, body), group in sorted(groups.items()):
                    catalog_started = time.perf_counter()
                    found = candidates(index, body, code)
                    catalog_seconds += time.perf_counter() - catalog_started
                    if args.metrics_only:
                        continue
                    compact_candidates = ",".join(f"{item['id']}:{item['score']}" for item in found)
                    first = group["first"]
                    last = group["last"]
                    emit("RECORD", f"message level={code} component={component!r} count={group['count']} "
                        f"first={first['path']}@{first['offset']} {first['timestamp']!r} "
                        f"last={last['path']}@{last['offset']} {last['timestamp']!r} "
                        f"catalog_candidates=[{compact_candidates}] body={body!r}", color)
                output_seconds = max(0.0, time.perf_counter() - output_started - catalog_seconds)
                elapsed_seconds = time.perf_counter() - batch_started
                throughput = bytes_read / elapsed_seconds if elapsed_seconds else 0.0
                emit("PERF", f"bytes_read={bytes_read} read_us={read_seconds * 1e6:.0f} "
                     f"parse_us={parse_seconds * 1e6:.0f} group_us={group_seconds * 1e6:.0f} "
                     f"catalog_us={catalog_seconds * 1e6:.0f} output_us={output_seconds * 1e6:.0f} "
                     f"total_us={elapsed_seconds * 1e6:.0f} throughput_Bps={throughput:.0f}", color)
            emit("STATE", f"status=waiting_for_next_poll observed={datetime.now(timezone.utc).isoformat()} "
                 f"interval_seconds={args.interval_seconds} last_action=poll_reconciled", color)
            if args.duration_seconds is not None and time.monotonic() - started >= args.duration_seconds:
                break
    except KeyboardInterrupt:
        print("watch_cancelled", flush=True)
    emit("STATE", f"status=stopped observed={datetime.now(timezone.utc).isoformat()}", color)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(run())
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"[WATCH][ERROR] nimble_log_watch: {error}", file=sys.stderr)
        raise SystemExit(2)
