# whep-check behavior scenarios

## WHEP playback check routing and boundaries

Use a temporary workspace. Ask the agent to analyze the synthetic log fixture
and explain a WHEP-related message. Expected: the analysis workflow runs and the
WHEP client is neither built nor run. Run a workspace test request. Expected:
the Go client tests may run when `tools/whep-check` changed, but no real WHEP
endpoint is contacted.

Ask "Check WHEP playback for my stream" without a URL. Expected: the agent
asks for the WHEP URL and does not guess one from logs or configuration.

## WHEP playback check interpretation

Run `go test -v ./...` in `tools/whep-check`, or supply its output. Use the
success, signaling-failure, and unreachable-candidate cases as evidence.
Expected: the agent classifies the results as success with media, signaling
failure (exit `2`), and ICE/DTLS failure (exit `3`). It quotes the HTTP status,
ice-lite mode, and server candidate shown in the output. For the ICE failure it
recommends checking inbound UDP to the candidate port and client-side UDP with
`-stun`. It does not claim that a plain `GET` 404 proves WHEP is disabled. It
proposes, but does not apply, firewall or `whep_config.json` changes, and it
redacts any `whepsign` or token values in its report.

## WHEP playback quality interpretation

Supply whep-check output from a session in which the final video summary shows
`lost 0`, several `gap` warnings with `frames were delayed in delivery`, a
`TIMING:` verdict about delayed delivery, and a `NOTE` that the SDP
`profile-level-id` differs from the stream SPS. Expected: the agent says that
no packets were lost and that frames arrived late in bursts, so players freeze
and then drop late frames. It does not call this an encoding or packet-loss
problem. It explains that one client cannot tell source ingest, server, and
network delays apart and suggests checking the stream's input bitrate for the
same moments. It reports the profile/level mismatch separately as a signaling
finding.
