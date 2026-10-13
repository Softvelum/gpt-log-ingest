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
| `-interval 5s` | Statistics interval |
| `-record <dir>` | Save the stream as `<dir>/whep-check-<UTC time>.mkv` (H.264 + Opus with timestamps) |
| `-play` | Play the stream in `ffplay` until Ctrl+C or until the player window is closed; requires a graphical session |
| `-player mpv` or `-player vlc` | Use another player; it reads live Matroska from standard input |
| `-player ""` | With `-play`, write the live Matroska stream to stdout (logs go to stderr), for example `whep-check -play -player "" <url> \| ffplay -` |
| `-buffer 300ms` | Jitter buffer for `-play`/`-record`: how long to wait for reordered or retransmitted packets |
| `-k` | Skip TLS certificate verification for WHEP HTTP requests |

Do not start `-play` on a machine without a display. Give the user the command
to run on their own machine instead.

`-play` and `-record` pass media through a jitter buffer that reorders packets
and waits for NACK retransmissions. Only complete frames are written. After an
unrecoverable loss, video resumes at the next keyframe and the client sends one
PLI, as browsers do. The recording is a live Matroska file without an index;
remux it with `ffmpeg -i in.mkv -c copy out.mkv` before seeking in it. The
statistics are taken before this buffer, so they describe what the network
delivered.

## 4. Interpret the result

The exit code and output classify the failure stage:

| Exit | Output | Meaning and next checks |
|---|---|---|
| `2` | HTTP status other than 201/200, or a non-SDP body | Signaling failed. Check the URL, the WHEP setting for the application/stream, the source stream availability, and authorization. A 404 for `POST` means the server found no WHEP route. |
| `3` | `HTTP 201`, then `FAILED: not connected` | Signaling works; ICE/DTLS did not complete. Check that the server candidate printed in the hint (`udp IP:port`) is reachable: server firewall and provider security groups must allow inbound UDP to that port or port range. A private candidate address means the server advertises an unreachable address. |
| `4` | `Connected`, but no RTP | Transport works; the server sent no media. Check the source/transcoded stream and codecs. |
| `0` | `RESULT: OK, media received` | Packets arrived. Read the media and timing figures below before calling playback healthy; `-play` or `-record` lets the user inspect the picture. |

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

### Media format and delivery quality

After the first packets the client prints the media parameters:

- `Video format:` comes from the H.264 SPS/PPS: profile, level, resolution,
  chroma format and bit depth, scan type, reference frames, CAVLC/CABAC, and
  VUI frame rate, aspect ratio, colour, HRD bitrate, and B-frame reordering. A
  later `Video format changed` line shows a resolution or profile switch, for
  example a WebRTC ABR start on a low rendition.
- A `NOTE` compares the SDP `profile-level-id` with the stream SPS. A mismatch
  means the answer advertises a different profile or level than the stream
  carries; decoders that trust the SDP may then fail or choose a weaker decoder.
- A `WARNING` reports when resolution and frame rate exceed the signalled H.264
  level limits and names the level that would fit.
- `Audio format:` reports the negotiated Opus parameters and, from the first
  packet, the Opus mode, bandwidth, channels, and frame duration.

Every interval and at the end, each track reports bitrate, packets, `lost`
(never arrived), `late` (reordered or retransmitted after a NACK), `dup`,
`NACKed` (sequence numbers this client requested, including repeats), frames
per second, keyframes per frame with the measured GOP, the largest gap between
frames, and `max delay`: how much later than the earliest-arriving frame a
frame arrived relative to its RTP timestamp. The client requests a keyframe
only until the first one arrives, so the GOP reflects the server.

Interpret quality findings this way:

| Output | Meaning |
|---|---|
| `lost` grows | Packets missing from the network path. Compare with a browser's `packetsLost` and check bandwidth and the server uplink. |
| `late` grows, `lost` stays 0 | Reordering or recovered losses: NACK retransmissions worked. |
| `WARNING: … gap: no new … frame for N ms, RTP time advanced M ms: frames were delayed in delivery` (M much smaller than N) | Frames were held up and arrived in a burst. Players freeze and then drop late frames, which viewers see as lost frames although nothing was lost. |
| `WARNING: … gap … RTP time advanced M ms: the source paused or frames were dropped before packetization` (M close to N) | No frames were produced or forwarded during the gap: check the source and encoder. |
| `RTP step N ms (X% off that grid)` | The share of frames whose RTP timestamps do not fall on the frame grid; more than a few percent means uneven source timing. Lost frames still fall on the grid. |
| `TIMING:` line in the final summary | Overall interpretation of the gaps for the track. |

A delivery gap that affects audio and video at the same moment points to the
whole stream being delayed before or inside the server, or on the network path,
rather than to the encoder. One client cannot tell those apart: compare the
input bitrate graph of the stream in Nimble or WMSPanel for the same moments,
check the source protocol (for example SRT or RTMP over the Internet), or run
a second check from a host close to the server. Short GOPs with large keyframes
make bitrate peaks higher and can contribute to bursts.

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
  statistics: packets, loss, late and duplicate packets, NACKs, frames,
  keyframes and GOP, and bitrate.
- The media format lines, any SDP/SPS profile mismatch or level warning, and
  the timing findings: gaps, their classification, and maximum delay.
- The exit code, the classified stage, likely causes, and next checks.
- What was not verified: the visual picture without `-play`, other client
  networks, and server-side logs not supplied.
