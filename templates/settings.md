# Local Nimble log settings

Copy this file to `.local/settings.md` or ask the agent to create that file
from your supplied directory. Replace the example directory with your own.
Paths here describe your machine, not paths required by this repository.

- Log directory: ~/Development/nimble/log
- Log timezone: unknown
- Include: nimble.log, nimble-*.log
- Exclude: access.log, advertisements.log, nimble_ut.log

An unknown timezone permits reporting log-local timestamps. Set the actual
server log timezone or offset when cross-timezone comparisons are needed.
Do not substitute your workstation's timezone without checking the logs.

Optional saved watch defaults can be added after you choose them: status heartbeat
interval for Rust (poll interval for Python),
duration or another stopping condition, and whether to include historical logs.
These settings do not start or schedule a watch.

Active watching recommends Rust/Cargo for event-driven reads and local grouping;
its heartbeat also reconciles missed filesystem notifications. Python 3 is the
polling fallback if you decline installing Rust. Both classify levels only from
exact `E`, `V`, `D`, and `I` prefix codes and emit tagged state and per-stage
performance output.

Workspace test mode and optional separate-instance launch details belong in
`.local/nimble-test.json`; see [the test workflow](../workflows/test-workspace.md).
Use `{"mode": "observe"}` for live or user-started installations. Analysis itself
never controls Nimble, regardless of the saved test mode.
