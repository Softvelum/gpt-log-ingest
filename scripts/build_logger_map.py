#!/usr/bin/env python3
"""Build the shared Rust/Python candidate index from logger/*.md."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LOGGER = ROOT / "logger"
OUTPUT = LOGGER / "search-index.json"
TOKEN_RE = re.compile(r"[A-Za-z0-9]+")


def field(text: str, name: str) -> str:
    match = re.search(rf"^{re.escape(name)}:\s*(.+)$", text, re.MULTILINE)
    return match.group(1).strip() if match else ""


def tokens(text: str) -> set[str]:
    return {item.lower() for item in TOKEN_RE.findall(text)}


def build() -> bytes:
    entries = []
    inverted: dict[str, list[int]] = {}
    for path in sorted(LOGGER.glob("log_*.md")):
        text = path.read_text(encoding="utf-8")
        first_line = text.splitlines()[0]
        title = (first_line[2:] if first_line.startswith("# ") else first_line).strip()
        pattern_match = re.search(
            r"^## Log pattern\s*\n\s*```[^\n]*\n(.*?)\n```",
            text,
            re.MULTILINE | re.DOTALL,
        )
        pattern = pattern_match.group(1).strip() if pattern_match else ""
        entry = {
            "id": path.stem,
            "title": title,
            "level": field(text, "Log level"),
            "component": field(text, "Component"),
            "pattern": pattern,
            "path": path.name,
        }
        index = len(entries)
        entries.append(entry)
        for token in tokens(f"{title} {entry['component']} {pattern}"):
            inverted.setdefault(token, []).append(index)
    payload = {
        "schema_version": 1,
        "entry_count": len(entries),
        "entries": entries,
        "terms": {term: inverted[term] for term in sorted(inverted)},
    }
    return (json.dumps(payload, ensure_ascii=False, separators=(",", ":")) + "\n").encode()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if logger/search-index.json differs from the Markdown catalog",
    )
    args = parser.parse_args()
    expected = build()
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_bytes() != expected:
            print(f"stale or missing generated map: {OUTPUT.relative_to(ROOT)}")
            return 1
        print(f"logger map is current ({json.loads(expected)['entry_count']} entries)")
        return 0
    OUTPUT.write_bytes(expected)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({json.loads(expected)['entry_count']} entries)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
