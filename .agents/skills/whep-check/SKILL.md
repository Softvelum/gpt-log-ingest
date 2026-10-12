---
name: whep-check
description: Use when a user asks to check WebRTC WHEP playback from a Nimble Streamer or other WHEP endpoint URL, diagnose why a WHEP player does not start, or watch or record a WHEP stream with the whep-check client.
---

# WHEP playback check

Read and follow the [shared WHEP check workflow](../../../workflows/check-whep.md).
Resolve this link from this skill file; the repository root is three directories
above its containing directory. The client source is in `tools/whep-check/`.

The check opens a real WHEP viewer session, so run it only on an explicit
request and only for a URL the user supplied. It never controls or reconfigures
Nimble. Ordinary log analysis and workspace tests do not run it; they follow the
[log analysis workflow](../../../workflows/analyze-logs.md) and the
[test workflow](../../../workflows/test-workspace.md).

Keep runs bounded, redact URL signatures and tokens in reports, and classify the
result by stage: signaling, ICE/DTLS, media. Before attributing an ICE failure to
the server, confirm that this client's network passes UDP. Present firewall and
Nimble configuration changes as proposals; do not apply them.

Example request: "Check WHEP playback of
https://media.example/live/stream/whep.stream and explain why it fails."

This is a repository skill; keep it in the clone with the linked workflow and
client source.
