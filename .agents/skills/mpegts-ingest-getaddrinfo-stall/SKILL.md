---
name: mpegts-ingest-getaddrinfo-stall
description: Use when Nimble Streamer MPEG-TS inputs show far less bitrate than the source or drop out periodically while the source and CPU are fine, when /proc/net/udp shows Nimble sockets with full rx_queue and drops, when a user asks to check a nimdiag archive, or to confirm or rule out mpeg2tscamera ingest threads blocked in DNS resolution (getaddrinfo) by a slow nameserver.
---

# MPEG-TS ingest stall from blocking getaddrinfo

Read and follow the
[shared ingest stall workflow](../../../workflows/mpegts-ingest-getaddrinfo-stall.md).
Resolve this link from this skill file; the repository root is three directories
above its containing directory. The tools are in
`tools/mpegts-ingest-getaddrinfo-stall/`: `nimdiag` (Rust collector and
analyzer), `python/ingest_stall.py` (analyzer without a build), and
`collect_dns_evidence.sh`.

Analyzing supplied archives, `/proc/net/udp` copies, DNS captures and logs
only reads files. Collecting on a Nimble host (nimdiag, the DNS evidence
script, strace) needs root and touches the live process, so run or propose it
only on an explicit request, and prefer giving the operator the commands. Never
change resolver, network or Nimble configuration; propose changes instead.
Ordinary log analysis and workspace tests do not run the collectors; they
follow the [log analysis workflow](../../../workflows/analyze-logs.md) and the
[test workflow](../../../workflows/test-workspace.md).

Report the verdict (`confirmed`, `probable`, `not-matched`, `no-drops`) with the
evidence chain and its sources. Ask for missing DNS evidence before you name the
resolver as the cause. Redact host names, addresses and URL tokens from user
data in reports.

Example request: "Our UDP multicast inputs show about 100 kbps in Nimble while
HLS inputs are fine. Here is the nimdiag archive and a tcpdump of port 53."

This is a repository skill; keep it in the clone with the linked workflow and
tools.
