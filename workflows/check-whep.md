# Check WHEP playback

Use this procedure when the user explicitly asks to check WebRTC WHEP playback
from a Nimble Streamer (or other WHEP) endpoint, or to explain why a WHEP
player does not start. It uses the `whep-check` client in
[tools/whep-check](../tools/whep-check/). Resolve paths below against this
repository's root, the parent of `workflows/`.

Unlike [log analysis](analyze-logs.md), this check sends network traffic: it
opens one ordinary WHEP viewer session per run. It never starts, stops,
restarts, signals, or reconfigures Nimble, and it is not part of log analysis
or [workspace tests](test-workspace.md). Observe-mode tests and ordinary log
analysis must not run it.

## 1. Confirm the request and endpoint

Run a check only on an explicit request for this kind of check. The user must
supply the WHEP URL, for example
`https://media.example/live/stream/whep.stream`, or ask you to use one they
supplied earlier. Do not guess endpoint URLs from logs, configuration files, or
hostnames. Check only endpoints the user operates or that they ask you to check
on their behalf.

Treat URL query parameters and bearer tokens as secrets. Pass a token through
`-token` instead of a command line that will be shown in a report. Redact
signatures and tokens in reports, as the analysis workflow does for excerpts.

A plain `GET` of a WHEP URL normally returns HTTP 404 or 405, because WHEP
playback starts with a `POST` of an SDP offer. A 404 from `wget` or a browser
address bar alone does not show that WHEP is disabled.

## 2. Build the client

The client requires Go 1.22 or later. Build it once from the repository root:

```bash
go build -C tools/whep-check -o whep-check .
```

If Go is unavailable, explain that and offer the build commands for a machine
where Go is installed. Cross-compilation produces a single static binary. For
example, for a Windows machine with a graphical session:

```bash
CGO_ENABLED=0 GOOS=windows GOARCH=amd64 go build -C tools/whep-check -o whep-check.exe .
```

## 3. Run a bounded check

```bash
tools/whep-check/whep-check -duration 20s <whep-url>
```

Keep runs bounded: the default receives media for 20 seconds after connecting
and fails ICE/DTLS after 15 seconds (`-ice-timeout`). Repeat a run only when
the user asks or when a single additional run separates two hypotheses. Each
run deletes its WHEP resource on exit. The client prints UTC timestamps.

Useful options:

| Option | Purpose |
|---|---|
| `-v` | ICE, DTLS, and peer-connection debug logs from the WebRTC stack |
| `-stun stun:<host>:<port>` | Gather a server-reflexive candidate to prove that outbound UDP leaves this client network |
| `-record <dir>` | Save `video.h264` and `audio.ogg` for later inspection |
| `-play` | Forward media to `ffplay` through a local SDP file until Ctrl+C; requires a graphical session |
| `-player ""` | With `-play`, only write the SDP file and print the player command |
| `-k` | Skip TLS certificate verification for WHEP HTTP requests |

Do not start `-play` on a machine without a display. Give the user the command
to run on their own machine instead.

## 4. Interpret the result

The exit code and output classify the failure stage:

| Exit | Output | Meaning and next checks |
|---|---|---|
| `2` | HTTP status other than 201/200, or a non-SDP body | Signaling failed. Check the URL, the WHEP setting for the application/stream, the source stream availability, and authorization. A 404 for `POST` means the server found no WHEP route. |
| `3` | `HTTP 201`, then `FAILED: not connected` | Signaling works; ICE/DTLS did not complete. Check that the server candidate printed in the hint (`udp IP:port`) is reachable: server firewall and provider security groups must allow inbound UDP to that port or port range. A private candidate address means the server advertises an unreachable address. |
| `4` | `Connected`, but no RTP | Transport works; the server sent no media. Check the source/transcoded stream and codecs. |
| `0` | `RESULT: OK, media received` | Packets and, for H.264, keyframes arrived. It does not prove that the picture looks correct; `-play` or `-record` lets the user inspect the media. |

For an ice-lite server (`a=ice-lite` in the answer), the client always sends
the first connectivity check. A stateful server firewall therefore needs an
inbound rule for new UDP packets to the candidate port(s); replies use the
established flow. A stateless filter also needs the matching outbound rule
for the same source port(s).

Before blaming the server for exit `3`, check this client's network: run once
with `-stun stun:stun.l.google.com:19302`, or another STUN server the user
prefers. A `typ srflx` local candidate proves that outbound UDP leaves this
network. If no server-reflexive candidate appears, the client network may
block UDP, and the result does not establish a server-side fault.

For Nimble, the WHEP media UDP ports are configured in
`/etc/nimble/whep_config.json`. Use `PortMin`/`PortMax` for a range or
`ICEUDPMuxPort` for a single port, and `NAT1To1IPs` with
`NAT1To1CandidateType` for an instance behind 1:1 NAT. Nimble reads this file
at the start of each playback session. Candidate ports outside a configured
range show that the setting is not active. See the
[WHEP playback article](https://blog.wmspanel.com/2023/09/whep-webrtc-low-latency-playback-nimble.html).
Prefer a port range that does not overlap the host's ephemeral port range
(`net.ipv4.ip_local_port_range`), and do not reuse a port that is already used
by HTTP/3, WHIP ingest, or another listener. Present configuration and firewall
changes as proposals for the user; never apply them yourself.

When the user also supplies Nimble logs, correlate the check through the
[analysis workflow](analyze-logs.md). Match the WHEP session, socket, and
routing messages with catalog descriptions; for example,
[WHEP helper media read failed](../logger/log_d-c328f3ee32ba3db5.md) describes
a playback session removed after its helper connection closed.

## 5. Report

Report:

- The redacted URL, run time (UTC), and client location if relevant
  (for example, this agent host or the user's machine).
- HTTP status and timing, ice-lite mode, negotiated codecs, and the server
  candidate addresses and ports.
- ICE/DTLS outcome, the selected candidate pair when connected, and media
  statistics: packets, keyframes, and bitrate.
- The exit code, the classified stage, likely causes, and next checks.
- What was not verified: the visual picture without `-play`, other client
  networks, and server-side logs not supplied.
