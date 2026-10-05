---
name: nimble-logs
description: Use when a user asks to analyze Nimble Streamer logs, explain runtime messages, investigate a streaming incident, watch new events, or test this Nimble log workspace.
---

# Nimble logs

For a workspace test request or required commit-stage tests, follow the
[shared test workflow](../../../workflows/test-workspace.md). It runs repository
checks and checks logs in an explicit read-only or separate-instance test mode.
Never interact with a live installation; stop only a process this test started.

Read and follow the [shared analysis workflow](../../../workflows/analyze-logs.md).
Resolve this link from this skill file; the repository root is three directories
above its containing directory. Use that root for the workflow, catalog, templates,
and `.local/` files, even if the agent's working directory differs.

The workflow handles directory setup, saved user instructions, evidence-based
queries, and the limits of active-session watching. Rust/Cargo is recommended
for event-driven watching with heartbeat reconciliation; use the Python 3
polling fallback if the user declines Rust. Relay startup, heartbeat, batch,
and terminal status to the user, including quiet batches and the last checkpoint.
Keep the agent turn active and poll the running watcher tool session at each
heartbeat or returned batch; relay its output into the conversation. The tool
has no push channel into chat, so do not leave watcher output only in tool
results or claim continued reporting after the agent turn ends. Stop the helper
if the client cannot keep polling.
Use short waits no longer than the heartbeat, poll sooner during bursts, and
relay every returned batch before lengthy catalog work. Group only identical
messages; preserve distinct errors, exact levels, counts, short excerpts,
source files, timestamps when present, reader-supported positions, match
confidence, and the latest checkpoint. Use byte offsets when the reader
provides them; never invent them. Follow the user's excerpt-redaction
instructions. Report quiet heartbeats too. If tool output is clipped,
immediately label the report gap and partial counts, reconcile the bounded
source-log range from the last relayed checkpoint, and stop if it cannot be
recovered reliably. Keep helper coverage gaps separate from reporting gaps.
Relay parsing and waiting states, and explain any 32 KiB lag-recovery coverage
gap. Keep watcher status visibly tagged as `[WATCH]` and conversational findings
under `[ANALYSIS]`; state when the watcher is waiting, parsing, paused,
stopped, or has failed.
Severity comes only from the exact `E`, `V`, `D`, or `I` prefix code. For file
analysis, ask for a directory only when neither the request nor saved settings
supplies it.
Explain a supplied message directly without directory setup. Treat the
catalog as reference knowledge and the user's logs as observed evidence.

Example request: "Analyze the errors in ~/Development/nimble/log from the last
hour and explain which ones affected live/camera1."

This is a repository skill; keep it in the clone with the linked workflow and
catalog. End users do not need Superpowers or the Nimble source repository.
