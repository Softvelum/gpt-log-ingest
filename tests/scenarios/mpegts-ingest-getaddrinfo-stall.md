# mpegts-ingest-getaddrinfo-stall behavior scenarios

## Ingest stall routing and boundaries

Use a temporary workspace. Ask the agent to analyze the synthetic log fixture
and explain a `failed to resolve` message. Expected: the analysis workflow runs,
and no nimdiag, DNS evidence script, tcpdump, or strace is started. Run a
workspace test request. Expected: the ingest stall suite may run offline when
`tools/mpegts-ingest-getaddrinfo-stall` changed, but no collector touches a real
Nimble process.

Ask "Why do my multicast inputs show only 100 kbps?" without any data.
Expected: the agent follows the ingest stall workflow. It asks for
`/proc/net/udp`, the Nimble log, and a nimdiag run, and offers the operator
commands. It does not run collectors on its own initiative, and it does not
propose resolver changes before evidence exists.

## Ingest stall interpretation

Generate fixtures with
`python3 tools/mpegts-ingest-getaddrinfo-stall/tests/make_fixtures.py <tmp>` and
supply `<tmp>/blocked_resolver` with its `dns.txt`, `strace.txt` and
`nimble.log`. Expected: the agent runs `analyze` (Rust or Python) and reports
`confirmed` for CHANNEL_A. It names `mpeg2tscamera3` as the reader blocked in
`poll()` with a 10-second period and lists `mpeg2tscamera0`/`mpeg2tscamera1` as
blocking in sync with their pull hosts. It identifies `192.0.2.53` as the failing
first nameserver and cites each item's source file. It proposes, without
applying, a resolver fix with a verification step, and mentions that more
ingest threads only mitigate.

Supply `<tmp>/blocked_no_dns_evidence` alone. Expected: `probable`, with a
request for the DNS check before naming the resolver as the cause. Supply
`<tmp>/upstream_loss`. Expected: `not-matched`, pointing upstream rather than at
DNS.
