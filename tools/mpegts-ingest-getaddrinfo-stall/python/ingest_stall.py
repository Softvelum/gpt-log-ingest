#!/usr/bin/env python3
"""Offline analyzer for MPEG-TS ingest stalls caused by blocking getaddrinfo().

Reads a nimdiag result directory (plus optional DNS, strace and Nimble log text)
and checks the signature of a Nimble mpeg2tscameraN ingest thread that stops
reading its UDP inputs while it waits inside a synchronous DNS lookup.

  ingest_stall.py analyze DIR [--stream NAME] [--dns FILE] [--strace FILE]
                              [--nimble-log FILE] [--json]
  ingest_stall.py udp FILE [--json]       decode a /proc/net/udp copy

The Rust collector (nimdiag analyze / nimdiag udp) implements the same checks
and the same JSON report. Standard library only, Python 3.5+. Read-only: input
files are never modified and nothing is sent anywhere.
"""
import argparse
import collections
import glob
import json
import os
import re
import socket
import struct
import sys

VERSION = 1
POLL = ("7", "271")          # poll, ppoll (x86_64)
EPOLL = ("232", "281")       # epoll_wait, epoll_pwait (x86_64)
SYSCALL_NAMES = {"7": "poll", "271": "ppoll", "232": "epoll_wait", "281": "epoll_pwait",
                 "202": "futex", "35": "nanosleep", "45": "recvfrom", "0": "read"}
FULL_RXQ = 1048576           # a queue below 1 MiB is not treated as overflowing


# ---------------------------------------------------------------- helpers
def ms_of_day(t):
    """'YYYY-MM-DD HH:MM:SS.mmm' -> milliseconds since midnight."""
    hh, mm, rest = t[11:23].split(":")
    ss, _, ms = rest.partition(".")
    return ((int(hh) * 60 + int(mm)) * 60 + int(ss)) * 1000 + int((ms + "000")[:3])


def r2(x):
    return float("%.2f" % x)


def median(v):
    v = sorted(v)
    return v[len(v) // 2] if v else 0.0


def read_lines(path):
    with open(path, "rb") as f:
        return f.read().decode("utf-8", "replace").splitlines()


def read_jsonl(path):
    out = []
    if not os.path.isfile(path):
        return out
    for line in read_lines(path):
        line = line.strip()
        if line:
            try:
                out.append(json.loads(line))
            except ValueError:
                pass
    return out


# ---------------------------------------------------------------- /proc/net/udp
def hex_ip(h):
    return socket.inet_ntoa(struct.pack("<I", int(h, 16)))


def parse_proc_udp(lines):
    socks = []
    for line in lines:
        f = line.split()
        if len(f) < 13 or f[0] == "sl" or ":" not in f[1] or ":" not in f[2]:
            continue
        try:
            la, lp = f[1].split(":")
            ra, rp = f[2].split(":")
            socks.append({
                "local": "%s:%d" % (hex_ip(la), int(lp, 16)),
                "remote": "%s:%d" % (hex_ip(ra), int(rp, 16)),
                "remote_ip": hex_ip(ra), "remote_port": int(rp, 16),
                "state": f[3], "rxq": int(f[4].split(":")[1], 16),
                "inode": int(f[9]), "drops": int(f[12]),
            })
        except (ValueError, IndexError, struct.error, OSError):
            continue
    return socks


def dns_sockets(socks):
    """Connected UDP sockets to port 53 (st 01): glibc resolver queries in flight."""
    c = collections.Counter(s["remote_ip"] for s in socks
                            if s["remote_port"] == 53 and s["state"] == "01")
    return [{"server": k, "count": c[k]} for k in sorted(c)]


def cmd_udp(path, as_json):
    socks = parse_proc_udp(read_lines(path))
    limit = max([s["rxq"] for s in socks] or [0])
    full = [s for s in socks if s["rxq"] >= FULL_RXQ and s["rxq"] * 10 >= limit * 9]
    report = {"tool": "mpegts-ingest-getaddrinfo-stall", "version": VERSION, "kind": "udp",
              "sockets": len(socks), "full_queue": [s["local"] for s in full],
              "with_drops": [{"local": s["local"], "drops": s["drops"]} for s in socks if s["drops"] > 0],
              "dns_sockets": dns_sockets(socks)}
    if as_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return
    print("%-22s %-22s %3s %10s %12s %10s" % ("local", "remote", "st", "rx_queue", "drops", "inode"))
    for s in socks:
        mark = ""
        if s in full:
            mark = "  <- queue at limit"
        elif s["remote_port"] == 53 and s["state"] == "01":
            mark = "  <- DNS query in flight"
        print("%-22s %-22s %3s %10d %12d %10d%s" % (s["local"], s["remote"], s["state"], s["rxq"],
                                                    s["drops"], s["inode"], mark))
    print()
    print("sockets with queue at the limit (>= 1 MiB): %d" % len(full))
    print("sockets with drops: %d" % len(report["with_drops"]))
    print("connected sockets to :53: %s" % (", ".join("%s x%d" % (d["server"], d["count"])
                                                       for d in report["dns_sockets"]) or "none"))


# ---------------------------------------------------------------- text inputs
def parse_resolv(lines):
    ns, timeout, attempts, found = [], 5, 2, False
    for line in lines:
        f = line.split("#")[0].split()
        if not f:
            continue
        if f[0] == "nameserver" and len(f) > 1:
            ns.append(f[1])
            found = True
        elif f[0] == "options":
            for o in f[1:]:
                k, _, v = o.partition(":")
                if k == "timeout" and v.isdigit():
                    timeout = int(v)
                elif k == "attempts" and v.isdigit():
                    attempts = int(v)
    if not found:
        return None
    return {"nameservers": ns, "timeout": timeout, "attempts": attempts}


TCPDUMP_HEAD = re.compile(r"^(\d\d):(\d\d):(\d\d(?:\.\d+)?) IP ")
GETENT_REAL = re.compile(r"^real\s+(\d+)m([\d.]+)s")
RCODES = ("NoError", "FormErr", "ServFail", "NXDomain", "NotImp", "Refused")


def ip_port(tok):
    """'192.0.2.1.53' -> ('192.0.2.1', '53'); None for anything else."""
    f = tok.split(".")
    if len(f) != 5 or not all(x.isdigit() for x in f):
        return None
    return ".".join(f[:4]), f[4]


def tcpdump_dns(line):
    """Parse one (joined) tcpdump -nn [-vv] line of a DNS packet over IPv4 UDP."""
    tok = line.split()
    for i in range(1, len(tok) - 1):
        if tok[i] != ">" or not tok[i + 1].endswith(":"):
            continue
        a, b = ip_port(tok[i - 1]), ip_port(tok[i + 1][:-1])
        if not a or not b:
            continue
        j = i + 2
        if j < len(tok) and tok[j].startswith("["):        # [udp sum ok] / [bad udp cksum ...]
            while j < len(tok) and not tok[j].endswith("]"):
                j += 1
            j += 1
        if j >= len(tok):
            return None
        qid = tok[j].rstrip("+*%-$|")
        if not qid.isdigit():
            return None
        rcode, qtype, name = None, None, None
        for k in range(j + 1, min(len(tok), j + 6)):
            t = tok[k]
            if t in RCODES:
                rcode = t
            elif t.endswith("?") and t[:-1].isalnum() and t[:-1].isupper() and k + 1 < len(tok):
                qtype, name = t[:-1], tok[k + 1]
                break
            elif t != "q:":
                break
        return {"src": a[0], "sport": a[1], "dst": b[0], "dport": b[1], "id": qid,
                "rcode": rcode, "qtype": qtype, "name": name}
    return None


def parse_dns_text(lines):
    """resolv.conf lines, `time getent` output and tcpdump -nn [-vv] text of port 53."""
    joined = []
    for line in lines:
        if line[:1] in (" ", "\t") and joined and TCPDUMP_HEAD.match(joined[-1]):
            joined[-1] += " " + line.strip()
        else:
            joined.append(line)
    queries, replies, getent = {}, [], []
    for line in joined:
        m = GETENT_REAL.match(line.strip())
        if m:
            getent.append(r2(int(m.group(1)) * 60 + float(m.group(2))))
            continue
        h = TCPDUMP_HEAD.match(line)
        if not h:
            continue
        q = tcpdump_dns(line)
        if not q:
            continue
        t = int(h.group(1)) * 3600 + int(h.group(2)) * 60 + float(h.group(3))
        if q["dport"] == "53":
            queries[(q["dst"], q["sport"], q["id"])] = {"t": t, "server": q["dst"], "answered": False}
        elif q["sport"] == "53":
            replies.append({"t": t, "server": q["src"], "key": (q["src"], q["dport"], q["id"]),
                            "rcode": q["rcode"] or "NoError"})
    per = collections.OrderedDict()

    def srv(name):
        if name not in per:
            per[name] = {"server": name, "queries": 0, "replies": 0, "rcodes": {}, "lat": [], "unanswered": 0}
        return per[name]

    for q in queries.values():
        srv(q["server"])["queries"] += 1
    for r in replies:
        s = srv(r["server"])
        s["replies"] += 1
        s["rcodes"][r["rcode"]] = s["rcodes"].get(r["rcode"], 0) + 1
        q = queries.get(r["key"])
        if q and not q["answered"]:
            q["answered"] = True
            s["lat"].append(r["t"] - q["t"])
    for q in queries.values():
        if not q["answered"]:
            srv(q["server"])["unanswered"] += 1
    servers = []
    for name in sorted(per):
        s = per[name]
        lat = s.pop("lat")
        s["median_ms"] = r2(median(lat) * 1000)
        s["max_ms"] = r2(max(lat) * 1000) if lat else 0.0
        failed = sum(v for k, v in s["rcodes"].items() if k in ("ServFail", "Refused"))
        s["bad"] = s["queries"] >= 3 and (
            failed * 2 >= max(1, s["replies"]) and s["replies"] > 0
            or s["unanswered"] * 5 >= s["queries"]
            or s["median_ms"] >= 1000)
        servers.append(s)
    return {"servers": servers, "getent_s": getent, "resolv": parse_resolv(lines)}


STRACE_LINE = re.compile(r"^(?:\[pid\s+(\d+)\]\s+)?(?:(\d+)\s+)?(\d\d:\d\d:\d\d(?:\.\d+)?)\s+(.*)$")
STRACE_DUR = re.compile(r"<(\d+\.\d+)>\s*$")
STRACE_CONNECT = re.compile(r"^connect\((\d+), \{sa_family=AF_INET, sin_port=htons\((\d+)\), sin_addr=inet_addr\(\"([0-9.]+)\"\)")
STRACE_POLL = re.compile(r"^poll\(\[\{fd=(\d+), events=POLLIN\}\], 1, (\d+)")


def parse_strace(lines):
    """Blocking resolver polls: connect(fd, :53) followed by poll([fd], 1, T) lasting >= 1 s."""
    fd_server, pending, blocks = {}, {}, []
    for line in lines:
        m = STRACE_LINE.match(line.strip())
        if not m:
            continue
        pid = m.group(1) or m.group(2) or "0"
        body = m.group(4)
        c = STRACE_CONNECT.match(body)
        if c:
            if c.group(2) == "53":
                fd_server[(pid, c.group(1))] = c.group(3)
            else:
                fd_server.pop((pid, c.group(1)), None)
            continue
        p = STRACE_POLL.match(body)
        if p and int(p.group(2)) >= 1000:
            server = fd_server.get((pid, p.group(1)))
            if not server:
                continue
            if "<unfinished" in body:
                pending[pid] = server
                continue
            d = STRACE_DUR.search(body)
            if d and float(d.group(1)) >= 1.0:
                blocks.append((server, float(d.group(1))))
            continue
        if body.startswith("<... poll resumed>") and pid in pending:
            server = pending.pop(pid)
            d = STRACE_DUR.search(body)
            if d and float(d.group(1)) >= 1.0:
                blocks.append((server, float(d.group(1))))
    for pid, server in pending.items():
        blocks.append((server, -1.0))   # still blocked when the trace ended
    per = collections.OrderedDict()
    for server, dur in blocks:
        e = per.setdefault(server, {"server": server, "count": 0, "max_s": 0.0, "unfinished": 0})
        e["count"] += 1
        if dur < 0:
            e["unfinished"] += 1
        else:
            e["max_s"] = max(e["max_s"], r2(dur))
    return [per[k] for k in sorted(per)]


NIMBLE_LINE = re.compile(r"^\[(\S+ \S+) P(\d+)-T(\d+)\] \[([^\]]+)\] (\w): (.*)$")
URL_HOST = re.compile(r"url='[a-zA-Z][a-zA-Z0-9+.-]*://([^/:'?]+)")
RESOLVE_HOST = re.compile(r"failed to resolve(?: ip address from| host=)? ?'?([^'\s]+)")


def parse_nimble_log(lines, pid):
    rows = []
    for line in lines:
        m = NIMBLE_LINE.match(line)
        if m:
            rows.append((m.group(2), m.group(4), m.group(6)))
    pids = set(r[0] for r in rows)
    if pid and pid in pids:
        rows = [r for r in rows if r[0] == pid]
        used_pid = pid
    else:
        used_pid = None
    hosts = collections.defaultdict(collections.Counter)
    failures = collections.Counter()
    for _, thread, msg in rows:
        u = URL_HOST.search(msg)
        if u:
            hosts[thread][u.group(1)] += 1
        if "failed to resolve" in msg:
            r = RESOLVE_HOST.search(msg)
            failures[r.group(1) if r else "?"] += 1
    return {"pid": used_pid, "lines": len(rows),
            "thread_hosts": {t: sorted(hosts[t]) for t in sorted(hosts)},
            "resolve_failures": {k: failures[k] for k in sorted(failures)}}


# ---------------------------------------------------------------- nimdiag directory
def load_dir(d):
    timeline = read_jsonl(os.path.join(d, "timeline.jsonl"))
    if not timeline:
        raise SystemExit("error: %s has no timeline.jsonl (not a nimdiag result directory?)" % d)
    streams = collections.OrderedDict()
    sysrec = []
    for r in timeline:
        if r.get("type") == "stream":
            streams.setdefault(r["name"], []).append(r)
        elif r.get("type") == "sys":
            sysrec.append(r)
    threads = read_jsonl(os.path.join(d, "threads.jsonl"))
    events = []
    for f in sorted(glob.glob(os.path.join(d, "events", "event_*.json"))):
        try:
            with open(f) as fh:
                events.append(json.load(fh))
        except ValueError:
            continue
    env = os.path.join(d, "env")
    snaps = {}
    for tag in ("start", "end"):
        p = os.path.join(env, "proc_net_udp_%s.txt" % tag)
        if os.path.isfile(p):
            snaps[tag] = dns_sockets(parse_proc_udp(read_lines(p)))
    resolv = None
    p = os.path.join(env, "resolv.conf.txt")
    if os.path.isfile(p):
        resolv = parse_resolv(read_lines(p))
    return streams, sysrec, threads, events, snaps, resolv


def stream_stats(rows):
    n = len(rows)
    rxq = [r["sock"]["rxq"] for r in rows]
    drops = [r["sock"]["drops_d"] for r in rows]
    rxq_max = max(rxq) if rxq else 0
    st = {
        "name": rows[0]["name"], "group": rows[0]["group"], "seconds": n,
        "drops": sum(drops), "drop_seconds": sum(1 for x in drops if x > 0),
        "rxq_max": rxq_max,
        "full_seconds": sum(1 for q in rxq if rxq_max >= FULL_RXQ and q * 10 >= rxq_max * 9),
        "wire_mbps": r2(sum(r["mbps"] for r in rows) / max(1, n)),
        "wire_pkts": sum(r["pkts"] for r in rows),
        "data_seconds": sum(1 for r in rows if r["pkts"] > 0),
        "cc_err": sum(r["cc_err"] for r in rows),
        "rtp_seq_err": sum(r["rtp_seq_err"] for r in rows),
        "sync_err": sum(r["sync_err"] for r in rows),
    }
    st["wire_clean"] = (st["sync_err"] == 0 and st["data_seconds"] * 100 >= n * 95
                        and (st["cc_err"] + st["rtp_seq_err"]) * 100 <= st["drops"])
    # drain-to-drain period: queue at the limit, then empty one second later
    drains = []
    if rxq_max >= FULL_RXQ:
        for i in range(1, n):
            if rxq[i - 1] * 10 >= rxq_max * 9 and rxq[i] * 10 <= rxq_max:
                drains.append(i)
    iv = [b - a for a, b in zip(drains, drains[1:])]
    st["drains"] = len(drains)
    st["period_s"] = 0
    st["period_share"] = 0.0
    if len(iv) >= 3:
        c = collections.Counter(iv)
        p = sorted(c.items(), key=lambda kv: (-kv[1], kv[0]))[0][0]
        ok = sum(1 for x in iv if p >= 2 and abs(x - p * max(1, int(round(float(x) / p)))) <= 1)
        st["period_s"] = p
        st["period_share"] = r2(float(ok) / len(iv))
    st["periodic"] = st["period_s"] >= 2 and st["period_share"] >= 0.8
    return st


def reader_thread(rows, rxq_max, threads):
    """The thread that reads this socket: many wakeups per second while the queue is
    drained, (almost) none while it is full. threads.jsonl rolls up per second on the
    sampler's own clock and may trail the socket counters by one record, so lags 0
    and +1 are both tried."""
    nz = sorted(r["sock"]["drops_d"] for r in rows if r["sock"]["drops_d"] > 0)
    if not nz or rxq_max < FULL_RXQ:
        return None
    full_rate = nz[len(nz) // 2]
    label = []
    for r in rows:
        dd, q = r["sock"]["drops_d"], r["sock"]["rxq"]
        if dd == 0 and q * 2 < rxq_max:
            label.append(1)                         # read the whole second
        elif dd * 10 >= full_rate * 9 and q * 10 >= rxq_max * 9:
            label.append(0)                         # stalled the whole second
        else:
            label.append(None)                      # transition
    pos = dict((r["t"][:19], i) for i, r in enumerate(rows))
    series = collections.OrderedDict()
    for rec in threads:
        i = pos.get(rec["t"][:19])
        if i is None:
            continue
        for x in rec.get("threads", []):
            series.setdefault((x["name"], x["tid"]), {})[i] = x
    cands = []
    for (name, tid), s in series.items():
        for lag in (0, 1):
            read, stall = [], []
            for i, a in enumerate(label):
                x = s.get(i + lag)
                if a is None or x is None:
                    continue
                (read if a == 1 else stall).append(x)
            if len(read) < 5 or len(stall) < 5:
                continue
            mr = float(sum(x["vcsw_d"] for x in read)) / len(read)
            ms = float(sum(x["vcsw_d"] for x in stall)) / len(stall)
            if mr <= 0 or ms > 0.2 * mr:
                continue
            poll = sum(1 for x in stall if str(x.get("last_syscall")) in POLL)
            cands.append({"name": name, "tid": tid, "lag": lag, "read_vcsw": r2(mr), "stall_vcsw": r2(ms),
                          "poll_share": r2(float(poll) / len(stall)),
                          "camera": name.startswith("mpeg2tscamera")})
    pool = [c for c in cands if c["camera"]] or cands
    if not pool:
        return None
    pool.sort(key=lambda c: (-(c["read_vcsw"] - c["stall_vcsw"]), c["tid"], c["lag"]))
    best = dict(pool[0])
    best.pop("camera")
    return best


def blocked_threads(events, threads):
    """Threads that sit in poll()/ppoll() with no context switches for >= 1 s."""
    runs = {}
    sets = collections.defaultdict(set)
    names = {}
    source = "none"
    have_events = any(e.get("nimble_threads") for e in events)
    if have_events:
        source = "events"
        for ei, e in enumerate(events):
            samples = e.get("nimble_threads") or []
            times = [ms_of_day(s["t"]) for s in samples]
            step = median([b - a for a, b in zip(times, times[1:]) if b > a]) or 200
            cur = {}
            for si, s in enumerate(samples):
                seen = set()
                for x in s.get("threads", []):
                    tid = x["tid"]
                    names[tid] = x["name"]
                    seen.add(tid)
                    if str(x.get("syscall")) in POLL and x.get("vcsw_d", 0) == 0 and x.get("cpu_pct", 0) < 1:
                        cur[tid] = cur.get(tid, 0) + 1
                        sets[tid].add((ei, si))
                        r = runs.setdefault(tid, [0, 0])
                        r[0] = max(r[0], int(cur[tid] * step))
                    else:
                        if cur.get(tid, 0) > 0 and cur[tid] * step >= 1000:
                            runs[tid][1] += 1
                        cur[tid] = 0
                for tid in list(cur):
                    if tid not in seen:
                        if cur[tid] * step >= 1000:
                            runs[tid][1] += 1
                        cur[tid] = 0
            for tid, k in cur.items():
                if k * step >= 1000:
                    runs[tid][1] += 1
    elif threads:
        source = "seconds"
        cur = {}
        for si, rec in enumerate(threads):
            seen = set()
            for x in rec.get("threads", []):
                tid = x["tid"]
                names[tid] = x["name"]
                seen.add(tid)
                if str(x.get("last_syscall")) in POLL and x.get("vcsw_d", 0) == 0:
                    cur[tid] = cur.get(tid, 0) + 1
                    sets[tid].add((0, si))
                    r = runs.setdefault(tid, [0, 0])
                    r[0] = max(r[0], cur[tid] * 1000)
                else:
                    if cur.get(tid, 0) >= 2:
                        runs[tid][1] += 1
                    cur[tid] = 0
            for tid in list(cur):
                if tid not in seen:
                    if cur[tid] >= 2:
                        runs[tid][1] += 1
                    cur[tid] = 0
        for tid, k in cur.items():
            if k >= 2:
                runs[tid][1] += 1
    limit = 1000 if source == "events" else 2000
    out = [{"name": names[t], "tid": t, "max_stall_ms": runs[t][0], "stall_runs": runs[t][1]}
           for t in runs if runs[t][0] >= limit]
    out.sort(key=lambda x: (x["name"], x["tid"]))
    return out, sets, source


def busiest_camera(threads):
    """(name, average CPU %) of the busiest mpeg2tscamera thread in threads.jsonl."""
    acc = collections.OrderedDict()
    for rec in threads:
        for x in rec.get("threads", []):
            if x["name"].startswith("mpeg2tscamera"):
                a = acc.setdefault((x["name"], x["tid"]), [0.0, 0])
                a[0] += x.get("cpu_pct", 0)
                a[1] += 1
    if not acc:
        return None
    k = sorted(acc, key=lambda k: (-acc[k][0] / acc[k][1], k[1]))[0]
    return k[0], acc[k][0] / acc[k][1]


def host_counters(sysrec):
    cap = sum(r.get("capture_drops_d", 0) for r in sysrec)
    soft = sum(r.get("softnet_dropped_d", 0) for r in sysrec)
    nic = 0
    for r in sysrec:
        for _, d in sorted((r.get("nic") or {}).items()):
            nic += sum(v for k, v in d.items() if k not in ("rx_packets", "rx_bytes", "multicast"))
    live = max([r.get("dns_socks", 0) for r in sysrec] or [0])
    return {"capture_drops": cap, "softnet_dropped": soft, "nic_errors": nic,
            "clean": cap == 0 and soft == 0 and nic == 0}, live


def nimble_pid(streams):
    for rows in streams.values():
        for r in rows:
            for o in r["sock"].get("owners", []):
                c, _, p = o.rpartition(":")
                if p.isdigit():
                    return p
    return None


def analyze(d, stream=None, dns_file=None, strace_file=None, log_file=None):
    streams, sysrec, threads, events, snaps, resolv = load_dir(d)
    host, live_dns = host_counters(sysrec)
    blocked, sets, source = blocked_threads(events, threads)
    blocked_tids = set(b["tid"] for b in blocked)

    dns = {"resolv": resolv, "sockets": {"start": snaps.get("start", []), "end": snaps.get("end", []),
                                         "max_live": live_dns},
           "servers": [], "getent_s": [], "strace_blocking": [], "log": None}
    if dns_file:
        t = parse_dns_text(read_lines(dns_file))
        dns["servers"], dns["getent_s"] = t["servers"], t["getent_s"]
        if dns["resolv"] is None:
            dns["resolv"] = t["resolv"]
    if strace_file:
        dns["strace_blocking"] = parse_strace(read_lines(strace_file))
    if log_file:
        dns["log"] = parse_nimble_log(read_lines(log_file), nimble_pid(streams))
    sock_max = max([sum(x["count"] for x in dns["sockets"]["start"]),
                    sum(x["count"] for x in dns["sockets"]["end"]), live_dns])
    bad_servers = [s["server"] for s in dns["servers"] if s["bad"]]
    timeout = (dns["resolv"] or {}).get("timeout", 5)

    evidence, missing = [], []
    dns_evidence = False
    if sock_max > 0:
        dns_evidence = True
        evidence.append("up to %d connected UDP sockets to port 53 at once (resolver queries in flight)" % sock_max)
    if bad_servers:
        dns_evidence = True
        evidence.append("slow or failing nameserver(s) in the DNS capture: %s" % ", ".join(bad_servers))
    slow_getent = [g for g in dns["getent_s"] if g >= 1.0]
    if slow_getent:
        dns_evidence = True
        evidence.append("getent lookups took %s s" % ", ".join("%.2f" % g for g in slow_getent))
    if dns["strace_blocking"]:
        dns_evidence = True
        evidence.append("strace: %d resolver poll() call(s) blocked >= 1 s (%s)" % (
            sum(x["count"] for x in dns["strace_blocking"]),
            ", ".join(x["server"] for x in dns["strace_blocking"])))
    if dns["log"] and dns["log"]["resolve_failures"]:
        dns_evidence = True
        evidence.append("Nimble log reports failed resolution of %s" % ", ".join(sorted(dns["log"]["resolve_failures"])))
    if not dns_file:
        missing.append("DNS check: resolv.conf, `time getent ahosts <pull host>` and a 30 s tcpdump of port 53")
    if not strace_file:
        missing.append("optional: strace of the blocked thread ids (connect to :53 followed by a long poll)")
    if not log_file:
        missing.append("Nimble log with the thread name -> pull URL mapping")
    if source == "none":
        missing.append("thread samples (nimdiag could not see the Nimble process)")

    results = []
    names = [stream] if stream else list(streams)
    for name in names:
        if name not in streams:
            raise SystemExit("error: stream not found: %s" % name)
        rows = streams[name]
        st = stream_stats(rows)
        if not stream and st["drops"] == 0 and st["rxq_max"] < FULL_RXQ:
            continue
        reader = reader_thread(rows, st["rxq_max"], threads)
        st["reader"] = reader
        reasons = []
        reader_blocked = None
        if reader:
            for b in blocked:
                if b["tid"] == reader["tid"]:
                    reader_blocked = b
        if st["drops"] == 0:
            verdict = "no-drops"
            reasons.append("no socket drops during the run")
        elif not st["wire_clean"]:
            verdict = "not-matched"
            reasons.append("the wire itself shows loss or gaps (CC/RTP errors %d, sync errors %d, data in %d of %d s): "
                           "look upstream first" % (st["cc_err"] + st["rtp_seq_err"], st["sync_err"],
                                                     st["data_seconds"], st["seconds"]))
        elif not host["clean"]:
            verdict = "not-matched"
            reasons.append("drops before the socket (capture %d, softnet %d, NIC %d): host or NIC problem" % (
                host["capture_drops"], host["softnet_dropped"], host["nic_errors"]))
        else:
            reasons.append("kernel dropped %d datagrams at Nimble's socket in %d of %d s while the wire was clean" % (
                st["drops"], st["drop_seconds"], st["seconds"]))
            if st["periodic"]:
                reasons.append("queue fills and drains with a stable period of %d s (%d%% of cycles)" % (
                    st["period_s"], int(round(st["period_share"] * 100))))
            if reader:
                reasons.append("reader thread %s (tid %d) wakes %.0f times/s while the queue drains and %.0f/s "
                               "while it is full" % (reader["name"], reader["tid"], reader["read_vcsw"],
                                                     reader["stall_vcsw"]))
            if reader_blocked:
                reasons.append("the reader sits in poll() with no context switches for up to %.1f s" % (
                    reader_blocked["max_stall_ms"] / 1000.0))
                others = [b for b in blocked if b["tid"] != reader["tid"]]
                if others:
                    reasons.append("%d other thread(s) block the same way: %s" % (
                        len(others), ", ".join(b["name"] for b in others)))
            matches_timeout = reader_blocked is not None and reader_blocked["max_stall_ms"] >= timeout * 800
            if reader_blocked and dns_evidence and (st["periodic"] or matches_timeout):
                verdict = "confirmed"
            elif reader_blocked:
                verdict = "probable"
                if not dns_evidence:
                    reasons.append("no DNS evidence yet: collect it to confirm the resolver as the cause")
            elif reader is None and source == "none" and st["periodic"] and dns_evidence:
                verdict = "probable"
                reasons.append("no thread samples; periodic stalls plus DNS evidence fit the pattern")
            else:
                verdict = "not-matched"
                if reader and reader["poll_share"] < 0.5:
                    reasons.append("the reader is not blocked in poll() while the queue is full: "
                                   "look at CPU load or thread distribution instead")
                elif not reader and st["drains"] == 0:
                    busy = busiest_camera(threads)
                    reasons.append("the queue never drains: the reader is permanently behind, not periodically "
                                   "blocked; check the CPU use of the ingest threads%s" % (
                                       " (busiest: %s, %.0f%% CPU on average)" % busy if busy else ""))
                elif not reader:
                    reasons.append("no thread follows the reading phases")
        st["verdict"] = verdict
        st["reasons"] = reasons
        results.append(st)

    order = ["confirmed", "probable", "not-matched", "no-drops"]
    overall = min([s["verdict"] for s in results] or ["no-drops"], key=order.index)
    affected = [s["name"] for s in results if s["verdict"] in ("confirmed", "probable")]
    mapped = {}
    if dns["log"]:
        for b in blocked:
            h = dns["log"]["thread_hosts"].get(b["name"])
            if h:
                mapped[b["name"]] = h
    for b in blocked:
        if b["tid"] in sets and results and results[0].get("reader") and results[0]["reader"]["tid"] in sets:
            a = sets[b["tid"]]
            r = sets[results[0]["reader"]["tid"]]
            b["overlap_with_reader"] = r2(float(len(a & r)) / max(1, len(a | r)))
        else:
            b["overlap_with_reader"] = 0.0
    summary = {
        "confirmed": "an ingest thread is blocked in DNS resolution and stops reading its MPEG-TS inputs",
        "probable": "an ingest thread stops reading its inputs while blocked in poll(); DNS is the likely cause",
        "not-matched": "socket drops do not match the blocked-resolver pattern",
        "no-drops": "no socket drops were recorded",
    }[overall]
    return {
        "tool": "mpegts-ingest-getaddrinfo-stall", "version": VERSION, "kind": "analyze",
        "verdict": overall, "summary": summary, "affected_streams": affected,
        "streams": results, "threads": {"source": source, "blocked": blocked, "pull_hosts": mapped},
        "host": host, "dns": dns, "evidence": evidence, "missing": missing,
    }


def print_report(r):
    print("verdict: %s - %s" % (r["verdict"], r["summary"]))
    for s in r["streams"]:
        print()
        print("--- %s [%s]  %s" % (s["name"], s["group"], s["verdict"]))
        print("  wire %.2f Mbps, drops %d in %d/%d s, max rx_queue %d B, full %d s, period %s" % (
            s["wire_mbps"], s["drops"], s["drop_seconds"], s["seconds"], s["rxq_max"], s["full_seconds"],
            ("%d s (%d%%)" % (s["period_s"], int(round(s["period_share"] * 100)))) if s["period_s"] else "-"))
        for x in s["reasons"]:
            print("  * " + x)
    t = r["threads"]
    print()
    print("blocked threads (%s): %s" % (t["source"], ", ".join(
        "%s/%d %.1fs x%d" % (b["name"], b["tid"], b["max_stall_ms"] / 1000.0, b["stall_runs"])
        for b in t["blocked"]) or "none"))
    for name, hosts in sorted(t["pull_hosts"].items()):
        print("  %s pulls from %s" % (name, ", ".join(hosts)))
    d = r["dns"]
    if d["resolv"]:
        print("resolv.conf: nameservers %s, timeout %d s, attempts %d" % (
            " ".join(d["resolv"]["nameservers"]), d["resolv"]["timeout"], d["resolv"]["attempts"]))
    for s in d["servers"]:
        print("  dns %s: %d queries, %d replies %s, median %.1f ms, max %.1f ms, unanswered %d%s" % (
            s["server"], s["queries"], s["replies"], json.dumps(s["rcodes"], sort_keys=True), s["median_ms"],
            s["max_ms"], s["unanswered"], "  <- BAD" if s["bad"] else ""))
    if r["evidence"]:
        print()
        print("DNS evidence:")
        for e in r["evidence"]:
            print("  * " + e)
    if r["missing"]:
        print()
        print("not provided:")
        for e in r["missing"]:
            print("  - " + e)


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = ap.add_subparsers(dest="cmd")
    a = sub.add_parser("analyze", help="analyze a nimdiag result directory")
    a.add_argument("dir")
    a.add_argument("--stream")
    a.add_argument("--dns", help="text with resolv.conf, time getent and tcpdump -nn [-vv] port 53")
    a.add_argument("--strace", help="strace -tt -T output of the blocked thread ids")
    a.add_argument("--nimble-log", help="Nimble log for the thread -> pull URL mapping")
    a.add_argument("--json", action="store_true")
    u = sub.add_parser("udp", help="decode a /proc/net/udp copy")
    u.add_argument("file")
    u.add_argument("--json", action="store_true")
    args = ap.parse_args(argv)
    if args.cmd == "udp":
        cmd_udp(args.file, args.json)
    elif args.cmd == "analyze":
        r = analyze(args.dir, args.stream, args.dns, args.strace, args.nimble_log)
        if args.json:
            print(json.dumps(r, indent=2, sort_keys=True))
        else:
            print_report(r)
    else:
        ap.print_help()
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
