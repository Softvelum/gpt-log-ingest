# mpegts-ingest-getaddrinfo-stall

Tools for one Nimble Streamer failure pattern: an MPEG-TS ingest thread
(`mpeg2tscameraN`) stops reading its inputs while it waits inside a blocking
`getaddrinfo()` call. Nimble processes several MPEG-TS inputs in each of these
threads. When one input on the thread pulls from a host name and the first
nameserver in `/etc/resolv.conf` is slow or does not answer, the glibc resolver
waits in `poll()` for up to `timeout` seconds (default 5). During that time the
thread does not service its other sockets. A UDP input on the same thread then
fills its socket buffer within about a second, and the kernel drops the rest.
The input bitrate shown by Nimble collapses (for example to a few hundred kbit/s
instead of several Mbit/s), although the source and the network are fine and
the CPU is idle.

The agent procedure that uses these tools is
[workflows/mpegts-ingest-getaddrinfo-stall.md](../../workflows/mpegts-ingest-getaddrinfo-stall.md).

## Layout

```
rust/                      nimdiag (Cargo project): live collector + offline analyzer
python/ingest_stall.py     offline analyzer only, standard library (written for Python 3.5+, tested with 3.12)
collect_dns_evidence.sh    resolv.conf, getent timing, tcpdump of port 53, optional strace
testkit/                   fake nimble, DNS lab servers, MPEG-TS multicast sender
tests/                     synthetic fixtures, expected verdicts, parity check, live lab
```

| | `nimdiag` (Rust) | `python/ingest_stall.py` |
|---|---|---|
| `collect`: record a live run on the Nimble host | yes (root) | no |
| `analyze`: check a result directory | yes | yes, same checks and JSON |
| `udp`: decode a `/proc/net/udp` copy | yes | yes |
| Install | build once, one static binary | single file, no build |

Both analyzers only read files. They print a text report, or with `--json` a
report whose structure and values are identical between the two
implementations (`tests/compare_reports.py` checks this).

## Build

A recent stable Rust toolchain (tested with 1.98):

```bash
cargo build --release --locked --manifest-path tools/mpegts-ingest-getaddrinfo-stall/rust/Cargo.toml
# -> tools/mpegts-ingest-getaddrinfo-stall/rust/target/x86_64-unknown-linux-gnu/release/nimdiag
```

The binary is statically linked against glibc (`.cargo/config.toml`), so it
runs on old distributions (kernel 3.2+) without installing anything. Copy only
the binary to the Nimble host.

## nimdiag collect (on the Nimble host)

Read-only: nothing in Nimble or the system configuration is changed. It needs
root for the raw capture and for `/proc/<pid>` of the Nimble process.

```bash
sudo ./nimdiag collect --config streams.json --dry-run      # check config and interfaces
sudo ./nimdiag collect --config streams.json --duration 1200
```

`streams.json` is the stream list exported from Nimble / WMSPanel (`protocol`,
`ip`, `port`, `mip`). UDP multicast entries are measured; other entries are
ignored. `collect` is the default subcommand, so the older form
`nimdiag --config streams.json` still works. Run it in `screen` or `tmux`. At
the end it writes `nimdiag_<host>_<UTC time>.tar.gz` in the current directory.

What it records, side by side, with timestamps:

1. **Wire**: its own AF_PACKET capture, before any socket queue. Bitrate, packet
   rate, datagram sizes, bursts, gaps, RTP, MPEG-TS continuity (CC), sync, TEI,
   scrambling, PCR bitrate and jitter.
2. **Kernel**: for every Nimble socket bound to a configured group, `rx_queue`
   and drops, plus host UDP/softnet/NIC counters and the number of Nimble's
   connected UDP sockets to port 53 (resolver queries in flight).
3. **Nimble threads**: every 200 ms, per thread name, state, CPU, wait channel,
   current syscall and context switches.
4. **Flight recorder**: when a socket drops or its queue reaches 1 MiB, the last
   seconds of packets (pcap) and of thread samples are saved as an event.

The run ends with `summary.txt`, which contains the per-stream findings and the
output of `nimdiag analyze` for the run.

| Option | Default | Meaning |
|---|---|---|
| `--config FILE` | (required) | Nimble stream list JSON |
| `--duration SEC` | 1800 | Run time; Ctrl-C stops early and still writes all results |
| `--out DIR` | `.` | Where to put results |
| `--pre / --post SEC` | 10 / 5 | Event window before / after a trigger |
| `--ring-mb N` | 64 | Packet ring per stream for event pcaps (0 = no pcaps) |
| `--trigger-rxq BYTES` | 1048576 | Trigger when a Nimble socket queue reaches this size |
| `--trigger-cc` | off | Also trigger on CC errors seen on the wire |
| `--cooldown SEC` | 120 | Minimum time between events of one stream |
| `--max-events N` / `--max-disk-mb N` | 30 / 2048 | Limits for events and pcaps |
| `--event-pcap-all` | off | Event pcap contains all streams, not only the triggering one |
| `--pid PID` | auto | Nimble PID (default: the process named `nimble`) |
| `--no-join` | off | Do not hold IGMP membership for the configured groups |
| `--thread-ms MS` | 200 | Thread sampling interval |
| `--console SEC` | 5 | Console table interval |
| `--iface NAME` / `--mip IP` | from JSON | Override capture interface / default interface address |
| `--rcvbuf-mb N` | 64 | Capture socket buffer |
| `--nimble-conf FILE` | `/etc/nimble/nimble.conf` | Config copied into `env/` with credential-like values masked |
| `--no-tar` | off | Do not create the archive |

By default nimdiag joins each configured group on its `mip` with a socket bound
to an unrelated port, so traffic keeps reaching the host for the measurement even
if Nimble leaves a group; `--no-join` disables this.

Result directory:

```
nimdiag_<host>_<time>/
  summary.txt        totals, findings and the ingest stall check (start here)
  console.log        what was printed on screen
  timeline.jsonl     one record per stream per second + one "sys" record per second
  threads.jsonl      Nimble threads per second (ingest-related, busy or blocked)
  events.jsonl       index of triggered events
  events/            event_NNN_<stream>.json (200 ms thread samples) and .pcap
  env/               kernel, sysctls, NIC, IGMP, packages, /proc/net/udp at start and
                     end, resolv.conf, resolver setup, nimble.conf (masked)
  streams.json       copy of the input config
```

The archive contains host names, addresses and stream names. Treat it as
private data of the installation.

## DNS evidence (on the Nimble host)

```bash
sudo ./collect_dns_evidence.sh -H <pull host> [-H <pull host>]... -s 30 dns-evidence/
sudo ./collect_dns_evidence.sh -H <pull host> --strace dns-evidence/   # only with consent
```

It writes `dns.txt` with `/etc/resolv.conf`, `time getent ahosts <host>` for each
`-H`, and a 30-second `tcpdump -nn -vv` of port 53. With `--strace`, it samples
which Nimble threads are in `poll()`/`ppoll()` for about 5 seconds and then
traces only those thread ids (`connect`, `sendto`, `recvfrom`, `poll`) into
`strace.txt`. strace attaches to the running process, which slows the traced
threads while it runs. Tracing the whole process with `-f` would slow every
thread, which is why the script does not do it.

## nimdiag analyze / ingest_stall.py analyze (anywhere)

```bash
nimdiag analyze nimdiag_host_20260101_100000 \
    --dns dns-evidence/dns.txt --strace dns-evidence/strace.txt --nimble-log nimble.log
python3 tools/mpegts-ingest-getaddrinfo-stall/python/ingest_stall.py analyze <same arguments> [--json]
```

Extract the archive first (`tar xzf`). All inputs except the directory are
optional. Each one adds evidence, and the report lists what was not provided.

| Check | Data | Signature |
|---|---|---|
| Socket overflow | `timeline.jsonl` | Drops at Nimble's socket while the wire has no comparable CC/RTP/sync errors and the host has no capture/softnet/NIC drops |
| Periodicity | `timeline.jsonl` | Queue fills to the limit and drains with a stable period (80% of drain-to-drain intervals within 1 s of a multiple of the dominant one) |
| Reader thread | `threads.jsonl` | Thread with the largest drop in wakeups between seconds when the queue drains and seconds when it is full (to at most 20%; lag 0 or +1 record) |
| Blocked threads | `events/*.json`, else `threads.jsonl` | `poll`/`ppoll` (syscall 7/271) with no context switches and no CPU for at least 1 s (2 s from per-second data) |
| Resolver in flight | `env/proc_net_udp_*.txt`, `dns_socks` | Connected UDP sockets to port 53 |
| Resolver setup | `env/resolv.conf.txt` or `--dns` | Nameserver order, `timeout`, `attempts` |
| Nameserver health | `--dns` | Per server: queries, replies, rcodes, latency, unanswered. "Bad" = at least half SERVFAIL/REFUSED, at least 20% unanswered, or median at least 1 s |
| Lookup time | `--dns` | `time getent` of at least 1 s |
| Blocking lookups | `--strace` | `connect(fd, :53)` followed by `poll([fd], 1, T >= 1000)` lasting at least 1 s or unfinished |
| Thread to input | `--nimble-log` | Host names in `url='...'` per thread name (for the Nimble PID of the run) and `failed to resolve` messages |

Verdicts, per stream and overall (the strongest wins):

- `confirmed`: socket overflow, reader blocked in `poll()`, DNS evidence, and a
  stable period or stalls as long as the resolver timeout.
- `probable`: reader blocked in `poll()` without DNS evidence yet (collect it),
  or periodic stalls with DNS evidence but no thread samples.
- `not-matched`: drops with another explanation, such as loss on the wire, host
  drops, or a reader that is permanently behind (CPU). The reasons say which.
- `no-drops`: no socket drops in the run.

Exit status is 0 when the analysis ran, regardless of the verdict, and 2 for
usage or input errors.

## Tests

```bash
tools/mpegts-ingest-getaddrinfo-stall/tests/run_tests.sh          # offline, no privileges
sudo LIVE=1 tools/mpegts-ingest-getaddrinfo-stall/tests/run_tests.sh
```

The offline suite runs the Rust unit tests and generates six synthetic scenarios
(`tests/make_fixtures.py`): blocked resolver, blocked without DNS evidence,
per-second data only, slow reader, upstream loss and clean. It checks the
verdicts in `tests/expected.json` with both analyzers and compares their JSON
and text output. Fixtures use only documentation addresses (192.0.2.0/24,
198.51.100.0/24, 203.0.113.0/24, multicast 233.252.0.0/24) and `.example`
names.

`LIVE=1` adds `tests/live_lab.sh`, which reproduces the failure end to end in new
network, PID and mount namespaces. A fake `nimble` reads two multicast inputs and
resolves pull host names with the real glibc resolver on one input's thread. A
namespace-only `resolv.conf` lists a silent nameserver first.
`nimdiag collect` records the run and `collect_dns_evidence.sh --strace` captures
DNS. Both analyzers must report `confirmed` for that one input only. The host
network, its resolver configuration and any real Nimble process are not visible
inside the namespaces and are not touched.
