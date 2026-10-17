#!/usr/bin/env python3
"""Generate synthetic nimdiag result directories for the analyzer tests.

  make_fixtures.py OUTDIR

All data is invented: documentation addresses (RFC 5737, MCAST-TEST-NET
233.252.0.0/24), .example host names and generic channel names. Each scenario
directory has the layout nimdiag writes, plus optional dns.txt, strace.txt and
nimble.log inputs. tests/expected.json lists the verdict each one must get.
"""
import json
import os
import shutil
import socket
import struct
import sys
from datetime import datetime, timedelta

BASE = datetime(2026, 1, 1, 10, 0, 0)
PID = 4000
FULL = 2098880                      # rx_queue at the 2 MiB SO_RCVBUF limit
BAD_NS, GOOD_NS, HOST_IP = "192.0.2.53", "192.0.2.54", "198.51.100.11"
A = {"name": "CHANNEL_A", "group": "233.252.0.10:1234"}
B = {"name": "CHANNEL_B", "group": "233.252.0.11:1234"}
READER, PULL1, PULL2, OTHER = (4103, "mpeg2tscamera3"), (4100, "mpeg2tscamera0"), \
    (4101, "mpeg2tscamera1"), (4102, "mpeg2tscamera2")


def ts(sec):
    return (BASE + timedelta(seconds=sec)).strftime("%Y-%m-%d %H:%M:%S.%f")[:23]


def hexip(ip):
    return "%08X" % struct.unpack("<I", socket.inet_aton(ip))[0]


def stream_rec(sec, ch, inode, pkts=950, cc=0, rxq=0, drops=0, total=0):
    return {"t": ts(sec), "type": "stream", "name": ch["name"], "group": ch["group"],
            "pkts": pkts, "mbps": round(pkts * 1316 * 8 / 1e6, 3), "sizes": [[1316, pkts]] if pkts else [],
            "burst_1ms": 1, "burst_10ms": 10, "max_gap_ms": 1.1, "srcs": ["203.0.113.20"], "ttl": [60, 60],
            "frags": 0, "rtp": 0, "rtp_seq_err": 0, "ts_pkts": pkts * 7, "non188": 0, "sync_err": 0,
            "cc_err": cc, "cc_dup": 0, "tei": 0, "scrambled": 0, "null_pct": 5.0, "pcr_jitter_max_ms": 1.0,
            "pcr_mbps": 10.0, "pcr_disc": 0, "pids": 4,
            "sock": {"count": 1, "rxq": rxq, "drops_d": drops, "drops_total": total, "inodes": [inode],
                     "owners": ["nimble:%d" % PID]}}


def sys_rec(sec, dns=0, softnet=0):
    return {"t": ts(sec), "type": "sys", "udp": {"RcvbufErrors": 0}, "softnet_dropped_d": softnet,
            "softnet_squeeze_d": 0, "capture_pkts_d": 1900, "capture_drops_d": 0, "nimble_threads": 40,
            "nic": {"eth1": {"rx_packets": 1900, "rx_bytes": 2500000, "multicast": 1900, "rx_dropped": 0,
                             "rx_errors": 0, "rx_missed_errors": 0}},
            "dns_socks": dns}


def th(tid, name, sc, vcsw, cpu):
    return {"tid": tid, "name": name, "cpu_pct": cpu, "states": "SSSSS", "last_wchan":
            "poll_schedule_timeout" if sc == "7" else "ep_poll", "last_syscall": sc, "vcsw_d": vcsw, "nvcsw_d": 0}


def sample(tid, name, sc, vcsw, cpu):
    return {"tid": tid, "name": name, "state": "S", "cpu": 1, "cpu_pct": cpu, "syscall": sc,
            "wchan": "poll_schedule_timeout" if sc == "7" else "ep_poll", "vcsw_d": vcsw, "nvcsw_d": 0}


def proc_udp(entries):
    out = ["  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops"]
    for i, (lip, lport, rip, rport, st, rxq, inode, drops) in enumerate(entries):
        out.append("%4d: %s:%04X %s:%04X %s 00000000:%08X 00:00000000 00000000     0        0 %d 2 0000000000000000 %d" % (
            i, hexip(lip), lport, hexip(rip), rport, st, rxq, inode, drops))
    return "\n".join(out) + "\n"


def write(d, name, text):
    p = os.path.join(d, name)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w") as f:
        f.write(text)


def jsonl(d, name, rows):
    write(d, name, "".join(json.dumps(r, sort_keys=True) + "\n" for r in rows))


def common_env(d, dns_socks):
    mc = [("233.252.0.10", 1234, "0.0.0.0", 0, "07", FULL, 5001, 40000),
          ("233.252.0.11", 1234, "0.0.0.0", 0, "07", 0, 5002, 0)]
    dns = [(HOST_IP, 40100 + i, BAD_NS, 53, "01", 0, 6000 + i, 0) for i in range(dns_socks)]
    write(d, "env/proc_net_udp_start.txt", proc_udp(mc + dns))
    write(d, "env/proc_net_udp_end.txt", proc_udp(mc + dns))
    write(d, "env/resolv.conf.txt", "nameserver %s\nnameserver %s\nsearch example.net\n" % (BAD_NS, GOOD_NS))
    write(d, "env/nimble.conf.masked", "worker_threads = 4\npanel_uuid = ***masked***\n")
    write(d, "streams.json", json.dumps([
        {"id": "s1", "protocol": "udp", "ip": "233.252.0.10", "port": 1234, "description": "CHANNEL_A",
         "mip": "198.51.100.1"},
        {"id": "s2", "protocol": "udp", "ip": "233.252.0.11", "port": 1234, "description": "CHANNEL_B",
         "mip": "198.51.100.1"},
        {"id": "s3", "protocol": "hls", "description": "PULL_A", "url": "https://hls-a.example/live/index.m3u8"},
    ], indent=1))
    write(d, "summary.txt", "synthetic fixture\n")


def stall_phase(sec, period=10, read=3):
    """0..read-1 reading, read..period-1 stalled (thread in poll)."""
    return (sec % period) >= read


def blocked_resolver(d, seconds=120, dns_socks=3, events=True):
    """CHANNEL_A shares mpeg2tscamera3 with a pull input whose lookups block ~7 s of every 10 s."""
    common_env(d, dns_socks)
    tl, thr, total = [], [], 0
    for s in range(seconds):
        stalled = stall_phase(s)
        ph = s % 10
        rxq = 0 if not stalled else (1200000 if ph == 3 else FULL)
        drops = 950 if stalled and ph > 3 else 0
        total += drops
        tl.append(stream_rec(s, A, 5001, rxq=rxq, drops=drops, total=total))
        tl.append(stream_rec(s, B, 5002, pkts=380))
        tl.append(sys_rec(s, dns=dns_socks if stalled else 0))
        if stalled:
            reader = th(READER[0], READER[1], "7", 300 if ph == 3 else 0, 0.0)
            p1 = th(PULL1[0], PULL1[1], "7", 0, 0.0)
            p2 = th(PULL2[0], PULL2[1], "7", 0, 0.0)
        else:
            reader = th(READER[0], READER[1], "232", 950 + 7 * ph, 3.1)
            p1 = th(PULL1[0], PULL1[1], "232", 18 + ph, 0.2)
            p2 = th(PULL2[0], PULL2[1], "232", 21 - ph, 0.2)
        other = th(OTHER[0], OTHER[1], "232", 380 + (s % 3), 1.5)
        work = th(4001, "work1", "232", 30, 0.4)
        thr.append({"t": ts(s), "threads": [reader, other, p1, p2, work]})
    jsonl(d, "timeline.jsonl", tl)
    jsonl(d, "threads.jsonl", thr)
    os.makedirs(os.path.join(d, "events"), exist_ok=True)
    if not events:
        return
    idx = []
    for n, cyc in enumerate((1, 3, 5, 7, 9)):
        t0 = cyc * 10 + 4
        samples = []
        for k in range(-40, 11):                    # 200 ms samples from t0-8 s to t0+2 s
            x = t0 + k * 0.2
            stalled = (x % 10) >= 3.0
            if stalled:
                rows = [sample(READER[0], READER[1], "7", 0, 0.0), sample(PULL1[0], PULL1[1], "7", 0, 0.0),
                        sample(PULL2[0], PULL2[1], "7", 0, 0.0)]
            else:
                rows = [sample(READER[0], READER[1], "232", 190, 3.0), sample(PULL1[0], PULL1[1], "232", 4, 0.0),
                        sample(PULL2[0], PULL2[1], "232", 4, 0.0)]
            rows.append(sample(OTHER[0], OTHER[1], "232", 76, 1.5))
            samples.append({"t": (BASE + timedelta(seconds=x)).strftime("%Y-%m-%d %H:%M:%S.%f")[:23],
                            "threads": rows})
        ev = {"id": n + 1, "t": ts(t0), "stream": A["name"], "group": A["group"],
              "reason": "socket drops +950", "window": [ts(t0 - 8), ts(t0 + 2)],
              "at_trigger": {}, "stream_seconds": [], "pcap": None, "nimble_threads": samples}
        name = "event_%03d_%s" % (n + 1, A["name"])
        write(d, "events/%s.json" % name, json.dumps(ev, indent=1, sort_keys=True))
        idx.append({"id": n + 1, "t": ts(t0), "stream": A["name"], "group": A["group"],
                    "reason": ev["reason"], "file": "events/%s.json" % name, "pcap": None})
    jsonl(d, "events.jsonl", idx)


def dns_text(d):
    lines = ["$ cat /etc/resolv.conf", "nameserver %s" % BAD_NS, "nameserver %s" % GOOD_NS,
             "options timeout:5 attempts:2", "", "$ time getent ahosts hls-a.example", "real\t0m5.006s", "",
             "$ timeout 30 tcpdump -i any -nn -vv -l port 53"]
    for i in range(12):
        t0 = 10 * 3600 + 5 * 60 + i * 2.5
        port, qid = 41000 + i, 20000 + i
        host = ("hls-a.example", "hls-b.example", "hls-c.example")[i % 3]

        def hm(t):
            return "%02d:%02d:%09.6f" % (int(t // 3600), int(t % 3600 // 60), t % 60)
        lines += ["%s IP (tos 0x0, ttl 64, id %d, offset 0, flags [DF], proto UDP (17), length 60)" % (hm(t0), i),
                  "    %s.%d > %s.53: [bad udp cksum 0x1 -> 0x2!] %d+ A? %s. (32)" % (HOST_IP, port, BAD_NS, qid, host)]
        if i % 4 != 3:
            t1 = t0 + 2.0 + (i % 3)
            lines += ["%s IP (tos 0x0, ttl 63, id %d, offset 0, flags [none], proto UDP (17), length 60)" % (hm(t1), 900 + i),
                      "    %s.53 > %s.%d: [udp sum ok] %d ServFail q: A? %s. 0/0/0 (32)" % (BAD_NS, HOST_IP, port, qid, host)]
        t2 = t0 + 5.0
        lines += ["%s IP %s.%d > %s.53: %d+ A? %s. (32)" % (hm(t2), HOST_IP, port + 500, GOOD_NS, qid + 500, host),
                  "%s IP %s.53 > %s.%d: %d 1/0/0 A 203.0.113.%d (48)" % (hm(t2 + 0.0008), GOOD_NS, HOST_IP,
                                                                        port + 500, qid + 500, 10 + i % 3)]
    write(d, "dns.txt", "\n".join(lines) + "\n")
    write(d, "strace.txt", "\n".join([
        "# threads seen in poll(): %d %d %d" % (READER[0], PULL1[0], PULL2[0]),
        "[pid  %d] 10:06:00.000100 connect(31, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr(\"%s\")}, 16) = 0 <0.000020>" % (READER[0], BAD_NS),
        "[pid  %d] 10:06:00.000200 sendto(31, \"...\", 31, MSG_NOSIGNAL, NULL, 0) = 31 <0.000030>" % READER[0],
        "[pid  %d] 10:06:00.000300 poll([{fd=31, events=POLLIN}], 1, 5000) = 0 (Timeout) <5.004812>" % READER[0],
        "[pid  %d] 10:06:05.005400 connect(31, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr(\"%s\")}, 16) = 0 <0.000020>" % (READER[0], GOOD_NS),
        "[pid  %d] 10:06:05.005600 poll([{fd=31, events=POLLIN}], 1, 5000) = 1 ([{fd=31, revents=POLLIN}]) <0.000700>" % READER[0],
        "[pid  %d] 10:06:05.007000 recvfrom(12, \"G...\", 18800, 0, NULL, NULL) = 1316 <0.000010>" % READER[0],
        "[pid  %d] 10:06:01.000000 connect(17, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr(\"%s\")}, 16) = 0 <0.000020>" % (PULL1[0], BAD_NS),
        "[pid  %d] 10:06:01.000100 poll([{fd=17, events=POLLIN}], 1, 5000 <unfinished ...>" % PULL1[0],
        "[pid  %d] 10:06:04.100000 <... poll resumed>) = 1 ([{fd=17, revents=POLLIN}]) <3.099900>" % PULL1[0],
    ]) + "\n")
    rows = []
    for k, (tid, name, host) in enumerate([(PULL1[0], PULL1[1], "hls-a.example"), (PULL2[0], PULL2[1], "hls-b.example"),
                                           (READER[0], READER[1], "hls-c.example")]):
        rows.append("[2026-01-01 09:%02d:00 P%d-T%d] [%s] E: media sequence reset for url='https://%s/live/index.m3u8' "
                    "prev=%d new=%d current=%d chunks=5" % (10 + k, PID, tid, name, host, 500 + k, 498 + k, 498 + k))
    rows.append("[2026-01-01 09:20:00 P%d-T%d] [%s] E: failed to resolve 'hls-b.example' h_errnop=-2" % (PID, PULL2[0], PULL2[1]))
    rows.append("[2025-12-31 20:00:00 P3000-T3001] [mpeg2tscamera0] E: media sequence reset for url='https://old.example/x.m3u8' "
                "prev=9 new=7 current=7 chunks=5")
    write(d, "nimble.log", "\n".join(rows) + "\n")


def slow_reader(d, seconds=120):
    """CHANNEL_A's thread is busy (CPU) and drops all the time: no poll() stalls, no period."""
    common_env(d, 0)
    tl, thr, total = [], [], 0
    for s in range(seconds):
        drops = 300 + (s * 37) % 50
        total += drops
        tl.append(stream_rec(s, A, 5001, rxq=FULL - (s % 5) * 1000, drops=drops, total=total))
        tl.append(stream_rec(s, B, 5002, pkts=380))
        tl.append(sys_rec(s))
        thr.append({"t": ts(s), "threads": [th(READER[0], READER[1], "running", 1 + s % 4, 99.0),
                                            th(OTHER[0], OTHER[1], "232", 380, 1.5)]})
    jsonl(d, "timeline.jsonl", tl)
    jsonl(d, "threads.jsonl", thr)
    os.makedirs(os.path.join(d, "events"), exist_ok=True)


def upstream_loss(d, seconds=120):
    """Loss before the host: CC errors on the wire are comparable to the socket drops."""
    common_env(d, 0)
    tl, thr, total = [], [], 0
    for s in range(seconds):
        drops = 20 if s % 15 == 0 else 0
        total += drops
        tl.append(stream_rec(s, A, 5001, pkts=900 if s % 7 else 0, cc=40 if s % 7 == 1 else 0,
                             rxq=0, drops=drops, total=total))
        tl.append(sys_rec(s))
        thr.append({"t": ts(s), "threads": [th(READER[0], READER[1], "232", 900, 2.0)]})
    jsonl(d, "timeline.jsonl", tl)
    jsonl(d, "threads.jsonl", thr)
    os.makedirs(os.path.join(d, "events"), exist_ok=True)


def clean(d, seconds=60):
    common_env(d, 0)
    tl, thr = [], []
    for s in range(seconds):
        tl.append(stream_rec(s, A, 5001))
        tl.append(stream_rec(s, B, 5002, pkts=380))
        tl.append(sys_rec(s))
        thr.append({"t": ts(s), "threads": [th(READER[0], READER[1], "232", 950, 3.0)]})
    jsonl(d, "timeline.jsonl", tl)
    jsonl(d, "threads.jsonl", thr)
    os.makedirs(os.path.join(d, "events"), exist_ok=True)


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: make_fixtures.py OUTDIR")
    out = sys.argv[1]
    if os.path.exists(out):
        shutil.rmtree(out)
    d = os.path.join(out, "blocked_resolver")
    blocked_resolver(d)
    dns_text(d)
    blocked_resolver(os.path.join(out, "blocked_no_dns_evidence"), dns_socks=0)
    blocked_resolver(os.path.join(out, "blocked_seconds_only"), events=False)
    slow_reader(os.path.join(out, "slow_reader"))
    upstream_loss(os.path.join(out, "upstream_loss"))
    clean(os.path.join(out, "clean"))
    print("fixtures written to %s" % out)


if __name__ == "__main__":
    main()
