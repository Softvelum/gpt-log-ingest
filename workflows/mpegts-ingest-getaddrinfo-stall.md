# Diagnose an MPEG-TS ingest stall caused by blocking getaddrinfo

Use this procedure when the user explicitly asks to investigate MPEG-TS inputs
whose bitrate in Nimble collapses or keeps dropping out while the source is
fine, to check a nimdiag archive, or to confirm or rule out a blocked resolver
in Nimble's ingest threads. The tools are in
[tools/mpegts-ingest-getaddrinfo-stall](../tools/mpegts-ingest-getaddrinfo-stall/README.md).
Resolve paths below against this repository's root, the parent of `workflows/`.

## Background

Nimble processes MPEG-TS inputs in `mpeg2tscameraN` threads, several inputs per
thread. Each thread runs an event loop. If an input on that thread pulls from a
host name, Nimble can resolve the name with the synchronous glibc
`getaddrinfo()` inside the same loop. glibc sends the query to the first
`nameserver` in `/etc/resolv.conf` and waits in `poll()` for up to `timeout`
seconds (default 5) before it tries the next one. While it waits, the thread
reads none of its other sockets:

1. A UDP input on that thread fills its socket buffer (often 2 MiB) within about
   one second at a few Mbit/s. The kernel drops everything after that
   (`/proc/net/udp` `drops`, `UdpRcvbufErrors`).
2. The lookup repeats with each pull reconnect, so the stall repeats with a
   stable period (10 s in the confirmed case). The input shows a fraction of its
   real bitrate, or reaches it briefly and drops to zero.
3. Every input on the same thread is affected. With few ingest threads, nearly
   all UDP inputs degrade at once. Spreading inputs over more threads
   (`mpeg2ts_camera_threads`) leaves only the inputs that share a thread with a
   pull input. The pull inputs themselves usually survive because they buffer
   segments.
4. The trigger is outside Nimble: a nameserver that answers slowly, returns
   SERVFAIL late, or does not answer. All hosts that share that resolver break
   at the same moment, independent of Nimble or kernel versions.

The confirmed case had UDP multicast inputs affected and HLS pull inputs
triggering the lookups. nimdiag measures only UDP multicast inputs. For other
affected inputs, use their bitrate and stalls in Nimble together with the thread
evidence.

## Boundaries

- Analyzing material the user supplies (an archive, text copies, logs) only
  reads files and can be done at any time on request.
- `nimdiag collect`, `collect_dns_evidence.sh` and strace run on the Nimble host
  as root. They read `/proc`, capture traffic, and nimdiag joins the configured
  multicast groups. strace attaches to Nimble threads and slows them while it
  runs. Run or propose these only on an explicit request. Prefer giving the user
  the commands to run on their host. Run them yourself only when the user asks
  you to and you are on that host with their authorization. Suggest `--strace`
  only when the user agrees to attach to the live process.
- Never change `resolv.conf`, network configuration, `nimble.conf`, or restart
  Nimble; present those as proposals. Log analysis
  ([analyze-logs.md](analyze-logs.md)) and
  [workspace tests](test-workspace.md) never run the collectors. Observe-mode
  tests keep their no-probe rule.
- Archives, `dns.txt`, `strace.txt` and logs contain host names, addresses,
  stream names and URLs with tokens. Keep them out of the repository (use
  `.local/` or the user's location) and redact them in reports as the analysis
  workflow does.

## 1. Triage what the user already has

- **`/proc/net/udp` copy:** run `python3 tools/mpegts-ingest-getaddrinfo-stall/python/ingest_stall.py udp FILE`.
  Queues at the buffer limit (`rx_queue` such as `0x2006C0`) with growing
  `drops` mean the application is not reading. A socket that keeps its inode
  after an input restart was never recreated. Connected sockets to port 53 are
  resolver queries in flight.
- **Nimble log:** find `failed to resolve` messages ([IPv4 hostname resolution
  failed](../logger/log_e-7b8fc2155ba47440.md)). A slow lookup that eventually
  succeeds logs nothing, so their absence proves nothing. Messages that carry
  `url='...'`, such as [HLS input sequence position
  reset](../logger/log_e-1d40410e729a8e93.md), map an `mpeg2tscameraN` thread to
  the pull input it serves. Use lines of the current Nimble process (`P<pid>`)
  only, because thread assignment changes on restart.
- **Symptoms that point elsewhere:** CC errors at the source, NIC or softnet
  drops, a busy CPU, or one input always dropping without a period. Use
  [analyze-logs.md](analyze-logs.md) for the log side.

If the triage fits the pattern, ask for a nimdiag run and DNS evidence.

## 2. Record a run with nimdiag (on the Nimble host)

Build once (see the tool README) and copy the static binary to the host. Then
the operator runs:

```bash
sudo ./nimdiag collect --config streams.json --dry-run
sudo ./nimdiag collect --config streams.json --duration 1200
```

Twenty minutes covers many stall cycles. The run writes
`nimdiag_<host>_<UTC>.tar.gz`; its `summary.txt` already contains the ingest
stall check. Ask for the whole archive, or at least the directory without
pcaps, when the user analyzes elsewhere.

## 3. Collect DNS evidence (on the Nimble host)

Give the host names of the pull inputs (from the stream list) as `-H`:

```bash
sudo ./collect_dns_evidence.sh -H <pull host> -s 30 dns-evidence/
sudo ./collect_dns_evidence.sh -H <pull host> --strace dns-evidence/   # with consent only
```

`time getent ahosts` close to the resolver `timeout`, a first nameserver with
SERVFAIL, late answers or no answers, and strace lines
`connect(fd, ...:53)` + `poll([{fd=...}], 1, 5000)` lasting seconds each
confirm the resolver as the cause. On hosts where `/etc/resolv.conf` points to a
local stub (`127.0.0.53`, systemd-resolved), the stub absorbs the upstream delay
and Nimble waits on the stub. Capture on all interfaces (`-i any`, the default)
to see both legs.

## 4. Analyze

```bash
tar xzf nimdiag_<host>_<UTC>.tar.gz
nimdiag analyze nimdiag_<host>_<UTC> --dns dns-evidence/dns.txt \
    [--strace dns-evidence/strace.txt] [--nimble-log nimble.log]
```

Without the Rust binary, run `python3 tools/mpegts-ingest-getaddrinfo-stall/python/ingest_stall.py analyze`
with the same arguments; it gives the same result. Add `--json` for the full
report.

- `confirmed`: drops at Nimble's socket on a clean wire, the reader thread
  blocked in `poll()` with no context switches, DNS evidence, and a stable
  period or stalls as long as the resolver timeout.
- `probable`: the thread evidence fits but DNS evidence is missing. Ask for
  step 3 before naming the resolver as the cause.
- `not-matched`: read the reasons. Wire loss means look upstream. Host drops
  mean NIC/softnet. A queue that never drains means a busy reader.
- `no-drops`: nothing to explain in this run. Ask whether the symptom was
  present during it.

Check the report's "not provided" list and do not claim evidence that was not
supplied.

## 5. Report

Lead with the verdict and the affected inputs. Then give the evidence chain,
each item with its source file:

1. Socket queue at the limit with drops, while the wire bitrate and continuity
   are clean and the CPU is idle.
2. Period of the fill/drain cycle.
3. The reader thread (name, tid): wakeups while the queue drains versus while it
   is full, and the longest `poll()` stall.
4. Other threads that block in sync, and the pull inputs they serve (from the
   Nimble log).
5. Resolver sockets in flight, `resolv.conf` order and options, and per-server
   results from the DNS capture.
6. strace excerpt, if collected.

State what was excluded (wire, NIC/softnet, CPU) and what remains unverified,
for example the exact pull event that triggers each lookup, or inputs that
nimdiag does not measure.

## 6. Propose remediation

Present these as proposals for the operator; do not apply them:

- Remove the failing nameserver or move working ones first. Configure it where
  the host manages it (`resolvconf`/`/etc/network/interfaces`, netplan,
  systemd-resolved or DHCP), because a hand edit of `/etc/resolv.conf` may be
  overwritten. glibc uses at most three `nameserver` lines.
- As mitigation, `options timeout:1 attempts:2` shortens each stall. A local
  caching resolver keeps lookups fast when one upstream fails.
- glibc before 2.26 reads `resolv.conf` only at process start; there, Nimble
  needs a restart to use the new resolver list (`env/resolver.txt` shows the
  glibc version).
- More ingest threads (`mpeg2ts_camera_threads`) only reduce how many inputs
  share a thread with a pull input; they do not remove the stall.
- Verify after the change: `time getent ahosts <pull host>` returns quickly,
  socket `drops` in `/proc/net/udp` stop growing, and a short nimdiag run reports
  `no-drops`.

The behavior was observed with Nimble Streamer 4.3. These tools do not verify
whether a given Nimble build resolves names on the ingest thread. Report the
evidence chain when asking Softvelum about a specific build.
