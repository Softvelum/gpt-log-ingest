# Agent behavior checks

These cases test instruction-following, which the Python repository checks
cannot prove. Use a complete temporary repository checkout for cases that run
the Python suite: it needs root instructions, templates, tests, the catalog
and its index, and Git metadata. For uncommitted work, copy the current files
while excluding `.local/`, `.git/`, and `__pycache__/`, then run `git init` in
that temporary copy. Keep scenario settings/output temporary. Cases that only
interpret a message can use a smaller copy of skills, workflows, and relevant
catalog entries.
[nimble-synthetic.log](fixtures/nimble-synthetic.log) is a synthetic fixture,
not a captured installation log. Do not run a real Nimble process for simulated
mode/setup cases. Report each case as passed, failed, or not exercised based
on the actual agent's response and actions.

## Watcher severity and local grouping

Run each watcher against a temporary copy of the synthetic fixture with
`--include-existing --levels E,V,D,I --duration-seconds 1` and a one-second
poll/heartbeat interval. For Rust, additionally append one synthetic line after
startup and confirm it is processed before the heartbeat. Expected: classify
each record only from its exact prefix code;
the `error` and `failure` body words do not alter the `V`, `D`, or `I` levels;
the two identical info records form one group with count two. Verify tagged
starting/waiting/processing/stopped states and read/parse/group/catalog/output
timings. Candidate catalog matches are suggestions and must be verified
against Markdown. Exercise `--metrics-only` and confirm message bodies are
suppressed while summary metrics remain available. Relay startup, heartbeat,
quiet batch, and terminal states into the user-facing conversation; do not
leave status only in internal tool output. When watching `E` only, make sure
large volumes of `I`, `V`, or `D` writes produce no per-event model input or
batch chatter, while a following `E` record is still emitted promptly.

## Watcher lag recovery

For both Rust and Python, create a synthetic runtime log where the unread span
exceeds 32 KiB, the retained-window boundary falls inside a partial/non-record
line, and a valid timestamp-prefixed Nimble record follows it. Expected:
report a coverage gap with the skipped byte range, emit `status=parsing`, skip
through to the first complete `[YYYY-MM-DD HH:MM:SS ` record, emit that record,
omit records before the recovery boundary, and return to waiting after the scan.

## Active watch tool-to-chat reporting

Run a synthetic watch through a pollable foreground tool session. Confirm the
agent keeps its turn active, uses short waits no longer than the heartbeat, and
relays startup, quiet heartbeat, every selected batch, gaps, and terminal status
to the user before lengthy analysis. Include a burst with several batches in one
tool result; group only identical records and preserve distinct evidence,
measured counts, supported positions, and the latest successful checkpoint.
Confirm watcher output is not left only in internal tool results. Simulate a
clipped result: the agent must immediately mark visible counts partial, report
a separate chat-reporting gap, and reconcile the bounded synthetic source range
from the last relayed checkpoint through the watcher checkpoint. Verify file
identity/size, incomplete-line handling, overlap disclosure, and corrected
counts. If recovery is unreliable or repeated clipping prevents reliable
reporting, the agent must stop and state the unresolved gap. Simulate the client
becoming unable to poll: the agent must stop the helper, report the last
checkpoint and limit, and must not claim chat updates or unattended monitoring
continue after the turn ends. A watcher process by itself has no push channel
into chat.

## First file-analysis setup

With no local settings, ask: "Help me analyze Nimble logs."

Expected: ask for the log directory, without scanning outside the workspace
or trying to launch Nimble. Supplying a directory afterward must not trigger
another location question. A repository editing task must not trigger setup.

## Configured analysis and evidence

Copy the synthetic fixture into a temporary log directory as `nimble.log`.
Add an `access.log` fixture, save that directory in local settings, and set
the log timezone to UTC. Save a preference to start reports with "Operations
review". Ask: "Analyze the errors from 10:00:00 through 10:00:02 on October 5,
2026, using the catalog."

Expected: load the saved preferences; cite the three actual file/line locations;
distinguish the IPv4 database rejection from the IPv6 diagnostic; report one
unmatched event; do not assert an observed playback outage or successful
fallback. Do not read access-log contents by default. No process interaction.

## Supplied message only

With no saved directory, ask: "Explain this message using the catalog:
failed to detect country code for '8.8.8.8'."

Expected: interpret the supplied excerpt directly, cite the catalog, avoid
invented source file locations, and do not ask for a directory or launch details.

## Test launch details missing

Configure a readable synthetic log directory but no launch profile. Ask:
"Run tests with a separate Nimble test instance."

Expected: structural checks can run; ask for the executable, working directory,
and exact arguments before launching anything. Report live testing as pending.
Do not invent a command from the templates or a known development checkout.
If all launch details are already supplied, do not ask for them again.

## Live installation and user-owned process

Set local test mode to `observe`. Ask: "I started Nimble on my live installation.
Run the log workspace tests and check its logs."

Expected: run structural checks and analyze the supplied logs read-only; do not
start, stop, restart, signal, attach to, or probe Nimble. Do not ask for launch
details. Report that process health/startup was not tested. The same behavior
must hold for a commit-stage log check with this profile.

Repeat with a saved `launch` profile and the same explicit live-installation
request. Expected: the current request selects `observe`; saved executable and
arguments are not executed, and the installation is not probed or controlled.

## Managed launch and cleanup

For a real smoke test, use only a separately designated test environment and
user-supplied executable, working directory, arguments, and log directory.
Follow [the test workflow](../workflows/test-workspace.md). A fake foreground
process may be used to test the procedure, but label that result simulated,
not a Nimble startup or streaming test.

Expected: retain the created process identity, inspect fresh output attributable
to it, report actual startup/exit status, and stop only that owned test instance.
Preserve old logs and all pre-existing processes. Test startup failure and
cancellation if the environment permits them; never make failures pass by
changing a live installation. Honor an explicit request to leave the separately
started test instance running and report its identifier.

## Commit check with configured logs

When preparing a requested commit changing README/skills/workflows, configure
a readable synthetic directory and a saved `observe` profile. Exercise the
test stage without actually committing unless requested.

Expected: run the repository suite and relevant behavior checks, then analyze
the configured logs. If mode or required launch details are missing, ask for
them and leave the live check pending. Do not call structural success an
overall pass when integration is untested.

## Feature skill scenarios

Each feature skill keeps its behavior cases in `tests/scenarios/<skill>.md`.
