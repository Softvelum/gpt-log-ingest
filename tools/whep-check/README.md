# whep-check

When a WebRTC WHEP player does not start, logs alone often cannot show why:
signaling may succeed while the media path fails. The `whep-check` skill uses a
small Go client in this directory that acts as a WHEP player and reports which
stage failed.

```text
/whep-check Check WHEP playback of https://media.example/live/stream/whep.stream
and explain why the player does not start.
```

The check opens one real viewer session per run, so the agent runs it only when
you ask and only for the URL you supply. It never changes Nimble or firewall
settings; it proposes changes instead. Log analysis and workspace tests never
run it. Build it with Go 1.22 or later:

```bash
go build -C tools/whep-check -o whep-check .
tools/whep-check/whep-check https://media.example/live/stream/whep.stream
```

| Exit code | Meaning |
|---|---|
| `0` | Connected and received media |
| `2` | WHEP signaling failed (HTTP status or answer) |
| `3` | Signaling worked, but ICE/DTLS did not connect: usually UDP to the server candidate is blocked |
| `4` | Connected, but no media arrived |

The output includes UTC timestamps, the server's ICE candidates (address and
UDP port), ICE/DTLS state, and the selected candidate pair. Once media flows it
prints the video and audio format (H.264 profile, level, resolution, frame rate
and colour from the SPS; Opus mode and frame size) and flags a mismatch with
the SDP `profile-level-id`. Periodic and final statistics show bitrate, lost,
late and duplicate packets, NACKs, frames, keyframes and GOP, and delivery gaps
classified as delayed delivery or source pauses. Add
`-stun stun:stun.l.google.com:19302` to confirm that your own network passes
UDP, or `-v` for ICE/DTLS debug output.

To watch the stream on a machine with a display and `ffplay`, run
`whep-check -play <url>` until Ctrl+C or until you close the window; `mpv` and
`vlc` work through `-player`. To save it, use `-record <dir>`, which writes a
Matroska file with H.264 and Opus. Both pass media through a jitter buffer that
waits for retransmissions and skips to the next keyframe after a loss. Static
binaries for another OS can be cross-compiled, for example
`CGO_ENABLED=0 GOOS=windows GOARCH=amd64 go build -C tools/whep-check -o whep-check.exe .`.
See [the WHEP check workflow](../../workflows/check-whep.md) for interpretation,
including Nimble's WHEP UDP port settings and firewall rules.

## Tests

Run the offline tests when changing the client. They use an in-process
WHEP server and do not contact real endpoints:

```bash
(cd tools/whep-check && go vet ./... && go test ./...)
```

With `ffmpeg` in `PATH`, an end-to-end test also decodes real H.264 and Opus
sent with impairments; without it that test is skipped, so report it as not run.
Never run the client against a real endpoint as part of workspace or commit
tests; [WHEP checks](../../workflows/check-whep.md) are separate, explicit
requests.
