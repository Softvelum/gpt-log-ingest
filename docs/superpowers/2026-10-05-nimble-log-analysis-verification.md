# Nimble log analysis verification

Date: 2026-10-05

Scope: the initial Markdown-only workspace described in the
[design](2026-10-05-nimble-log-analysis-design.md) and
[specification](2026-10-05-nimble-log-analysis-spec.md).

## Structural checks

- Both `nimble-logs` entry points passed the bundled Skill Creator validator
  for standard frontmatter, naming, and unfinished scaffold content.
- `cmp` confirmed byte-identical Codex and Claude entry points.
- Relative Markdown links in the new documents and relocated index resolved.
- `logger/README.md` was identical to the previous committed root index and
  retained all 3,410 message links, matching the existing catalog files.
- Existing message descriptions and `manifest.json` were unchanged.
- `git diff --check` passed. `git check-ignore` confirmed that settings,
  preferences, watch state, and report paths under `.local/` are ignored.
- Contributor instructions and watch limitations were reviewed against the spec.

## Isolated agent scenarios

Trials used temporary repositories and synthetic logs, not the user's actual
log contents. They exercise instruction retrieval and behavior, not native
client skill discovery. A baseline without the workflow omitted source line
citations and asked for unnecessary setup information.

| Scenario | Observed result |
|---|---|
| First analysis task, no directory configured | Asked only for the log directory; did not scan outside the workspace |
| Configured query | Read saved settings and preferences, applied the requested report prefix, and cited actual file/line evidence |
| Documented IPv4/IPv6 geolocation messages | Distinguished database rejection from diagnostic-only behavior; did not claim a playback outage or confirmed recovery |
| Unmatched synthetic message | Reported one unmatched event and left its cause and impact unresolved |
| Runtime directory also containing `access.log` | Excluded access-log contents from runtime-message analysis |
| Supplied directory, source timezone unknown | Did not repeat the directory question; saved initial setup and asked about the source log timezone when UTC conversion was needed |
| Explicit server log timezone UTC | Answered the requested interval without a setup question |
| Pasted message, no directory configured | Explained the supplied evidence with catalog references, without a directory question or invented file locations |

An independent documentation review found that the original draft forced a
pasted-message explanation through directory setup. The workflow, entry points,
README, and design/specification were corrected; a fresh agent trial verified
the catalog-only path. Initial setup persistence versus later temporary
directory overrides was also made explicit.

## Limits of this verification

- Native Codex and Claude Code skill discovery, client trust prompts, and
  different client versions were not tested end to end.
- Active-session monitoring, rotation, compressed input, truncation, incomplete
  trailing lines, and checkpoint resumption were documented and reviewed, not
  exercised through a live watcher.
- No unattended scheduler, runtime parser, helper script, or Git hook was added.
- The synthetic catalog subset had incomplete related-message coverage and
  inherited a parameter typo from an existing entry. Trial agents disclosed
  these limits; existing catalog content was preserved.

The changes were left in the working tree for review; no commit or publication
was performed as part of this task.

## Test-workflow extension verification

The extension added `tests/test_repository.py`, `tests/scenarios.md`, synthetic
fixtures, the shared `workflows/test-workspace.md`, and a local launch template.
The repository suite first failed for absent test routes and the launch template;
after implementation, all 8 checks passed.

Two fresh-context synthetic behavior cases were executed in complete temporary
repositories. Both passed their 8 structural checks. A separate launch request
with no command caused the expected executable/working-directory/arguments
question and retained integration as pending. An explicit live/user-started
request overrode a stale saved launch profile and read synthetic logs without
executing its saved command or interacting with a process. Saved reporting
preferences, source references, and unmatched-message handling were retained.

The user then started Nimble. The repository's `nimble-logs` skill was available
and read in this Codex session, and its test/analysis workflows were exercised
in `observe` mode on actual log files. The current-run scope comprised 251
physical lines, with 214 timestamped headers and zero error-level messages.
Catalog interpretation identified missing optional SRT bonding support and
eight successful panel transport results, while separating an older run's
CUDA error from the new startup. Additional appended lines were read from a
recorded offset without changing the source file or interacting with Nimble.

Machine-specific evidence and exact source locations are saved in ignored
`.local/reports/nimble-read-only-test.md`; the directory and observe mode are
saved locally. Real log payloads and local parameters are not checked in.
An independent review found no blocking defects in the testing extension.

At that point, managed real Nimble launch/cleanup, native Claude discovery,
playback/network probes, and exhaustive catalog matching remained untested.
The observed log check is not a managed-startup test or a full installation-health test.

## Managed real-instance verification

The user subsequently requested a separate startup/cleanup test and supplied
the executable, working directory, and an explicit empty argument list.
The launch profile was saved locally. The agent launched that exact foreground
command and retained its `Popen` child identity. It survived the 10-second
smoke-test interval and produced 268 attributable timestamped runtime headers.
There were no error/warning headers in the captured owned-process sample.
The optional SRT bonding diagnostic remained present.

Only the owned child received SIGTERM. It logged normal termination and
`server stopped`, then was reaped with exit status 0. No forceful agent signal,
service-management command, external process inspection, configuration change,
or network probe was used. The agent did not control a user-started instance.
The unchanged runtime-file identity and recorded starting offset allowed all
277 appended physical lines to be captured without truncation; PID prefixes
separated owned headers from other physical lines.

The 8 repository checks and `git diff --check` passed again.
Machine-specific evidence is in ignored `.local/reports/nimble-owned-test.md`
and its JSON snapshots. Startup/log/normal-termination smoke coverage is now
verified on real Nimble. Native Claude discovery, playback, full service health,
exhaustive child/resource cleanup, and complete catalog matching remain outside
this test's coverage.

## Responsive watcher and timing verification

The Rust watcher now uses filesystem notifications for immediate processing
and heartbeat reconciliation for missed notifications; access-only events are
ignored to avoid unnecessary file scans. Python remains the polling fallback.
Both implementations emit `[WATCH]` status and performance tags, ANSI color is
configurable, and metrics-only mode suppresses message content while retaining
parsing, grouping, and catalog timing.

Two Rust integration tests passed: appends were processed before the 10-second
heartbeat, and metrics-only output omitted the synthetic message body. Two
Python behavior tests passed for exact-level handling, repeat grouping, stage
timings, state labels, and metrics-only suppression. The full Python suite
passed (11 tests), the Rust suite passed (4 unit tests and 2 integration tests),
and the generated map remained current at 3,410 entries.

A 60-second release-helper check watched the configured Nimble log directory
from current file ends with `--metrics-only`; it displayed no message bodies or
candidate IDs. The watcher saw four runtime files, processed appended records
within seconds of filesystem notifications, and reported no coverage gaps.
Observed per-batch total time ranged from under 0.1 ms to about 7.2 ms.

The first 16 MiB backlog benchmark exposed full sorting of every catalog
candidate for each of 37,695 groups. Rust catalog lookup took 19.2 seconds and
the Python fallback took 20.0 seconds. Both implementations were changed to
select only the top three candidates before sorting; Python timing was also
adjusted so catalog lookup and output are reported as non-overlapping stages.
On a repeated run of the same current log set, Rust processed 16 MiB / 168,040
complete lines / 37,695 groups in 2.88 seconds total, with 2.73 seconds in
catalog lookup. The final Python run processed 16 MiB with the same line and
group counts in 17.35 seconds total, with 16.95 seconds in catalog lookup and
0.054 seconds in output. Rust was about 6 times faster on this workload. These
are local content-suppressed measurements, not guaranteed production timings
or AI interpretation latency. Both runs reported zero coverage gaps.

`cargo fmt --check` is unavailable because the installed Cargo toolchain lacks
the `fmt` subcommand; Rust changes were manually reviewed. No repository commit
was made.

## Exact-level watcher and search-index verification

The watcher extension adds a Rust incremental reader, a Python 3 fallback, and
`logger/search-index.json`, generated from all 3,410 catalog Markdown entries.
Both implementations were run against a temporary copy of the synthetic
fixture with `E,V,D,I` selected. Each recognized severity from the prefix even
when the body contained `error` or `failure`, grouped the duplicate info records
as count 2, and emitted matching byte offsets and candidate IDs. Rust reported
six groups; Python reported the same six groups. No installation logs were used.

The generated-map freshness check passed, all four Rust unit tests passed, and
all nine Python repository checks passed after map freshness was added to the
suite. `git diff --check` passed. `cargo fmt --check` could not run because the
installed Cargo toolchain does not include the `fmt` subcommand. Rust sources
were reviewed manually. This validates synthetic parsing and grouping, not
unattended monitoring, all rotation races, native client discovery, or full
catalog matching.

## Filtered output and lag-recovery verification

Rust and Python regression tests reproduce an unread span over 32 KiB whose
retained-window boundary lands inside an invalid partial line. Both watchers
reported the skipped byte range, ignored the earlier record, resumed at the
first complete Nimble timestamp-prefixed record, and emitted parsing state.
Rust tests passed (4 unit tests and 3 integration tests); the Python repository
suite passed (13 tests). The release build passed, the generated logger map was
current, skill entry points matched, ignored local paths were confirmed, and
`git diff --check` passed. A release watcher was restarted on the configured
log directory from current file ends; startup and the next 10-second parsing /
waiting heartbeat were observed. No installation log contents were included
in this verification note.

## Shared skill-source verification

The duplicated Claude skill directory was replaced with a symlink to the
canonical `.agents/skills/nimble-logs` directory. Repository tests verify the
symlink target, canonical frontmatter, and shared workflow links through both
client paths. Claude Code's official skill documentation confirms that project
skill folders may be symlinks and are loaded from their targets.

## Active watcher chat-reporting workflow

The new repository assertion first failed because the workflow did not state
that the agent must poll the watcher tool session, keep its turn active, and
relay output. After updating the workflow and its entry-point/docs, the focused
assertion passed. The full Python suite passed (14 tests), including the shared
workflow links and the new active-polling contract; `git diff --check` passed.
The behavior scenario for poll-to-chat relaying is documented but was not
simulated in a separate native Codex or Claude client. The live watcher remained
active during this documentation change; that confirms tool-session output can
be polled in this client, not an automatic push binding after a turn ends.

## WHEP playback check skill

`go vet ./...` and `go test -race ./...` passed in `tools/whep-check` with Go
1.22.2 and Pion WebRTC v4.0.16. The in-process ice-lite WHEP server tests showed:

- A connected session received synthetic H.264 and Opus RTP, detected H.264
  keyframes, recorded non-empty `video.h264` and `audio.ogg` files, and sent
  one `DELETE`.
- An HTTP 404 answer produced the signaling exit code.
- A server peer closed after answering produced the ICE timeout exit code, and
  the client still sent `DELETE`.

Unit tests cover H.264 keyframe detection for single NAL, STAP-A, and FU-A
payloads, and candidate address parsing. No real endpoint was contacted by the
tests.

The Python suite passed (17 tests), including frontmatter, symlink, and routing
checks for the new skill. The logger map was current, `git diff --check`
passed, and the client binary and `dist/` output are ignored. Claude Code
listed the `whep-check` skill through its `.claude/skills` symlink. Codex
discovery and the new behavior scenarios were not exercised in separate native
clients. Real-endpoint use is an explicit user task outside these tests.
