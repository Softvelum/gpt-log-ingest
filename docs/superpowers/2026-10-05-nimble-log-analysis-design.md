# Nimble log analysis workspace design

Date: 2026-10-05

## Purpose

Let a user clone this repository, start their own local AI agent, supply a
Nimble log directory, and ask questions using the message descriptions in
`logger/`. Users can add their own analysis instructions. Nimble itself needs
no changes, and users need neither its source checkout nor Superpowers.

The workspace supplies Markdown instructions and skills, a Rust watcher with
a Python 3 fallback, a generated catalog search index, and Python
standard-library repository checks. It supports existing-log queries and
best-effort monitoring during an active agent session. It does not ship a
scheduler, MCP server, or AI service.

The testing extension adds a shared test procedure: read-only checks on live
installations or a bounded smoke test of a separately configured foreground
Nimble instance. The agent asks for missing launch details and may stop only
the process it actually started. It never controls Nimble during ordinary
analysis or read-only/live-installation testing.

The WHEP playback extension adds a second repository skill, `whep-check`. It
is a Go WebRTC client and workflow that checks a user-supplied WHEP endpoint
by opening one bounded viewer session. It reports signaling, ICE/DTLS, and media
results, and it is never part of log analysis or workspace tests.

The behavioral contract is in the companion
[specification](2026-10-05-nimble-log-analysis-spec.md). Implementation steps
are in the [plan](plans/2026-10-05-nimble-log-analysis.md).

## Architecture

```mermaid
flowchart TD
    User[User question] --> Entry[Repository instructions or skill]
    Entry --> Workflow[Shared Markdown workflow]
    Settings[Local settings and user instructions] --> Workflow
    Logs[User log directory] --> Workflow
    Catalog[Message descriptions in logger] --> Workflow
    Catalog --> Index[Generated JSON term index]
    Logs --> Watcher[Rust watcher or Python fallback]
    Index --> Watcher
    Watcher --> Poll[Agent polls its tool session]
    Poll --> Workflow
    Workflow --> Report[Evidence and explanations]
    Workflow --> State[Local progress for an active watch]
```

The AI agent supplies file reading, search, execution, and repeated polling of
the watcher's tool session. All analysis decisions live in
`workflows/analyze-logs.md`. The skill entry points route to that document,
keeping the procedure independent of any one agent. The watcher has no direct
push channel to chat; visible reports require the active agent turn to poll and
relay the tool output. The agent drains short polling results before lengthy
catalog research, groups identical messages while retaining evidence and
checkpoints, and reports each result promptly. If the tool clips output, the
agent reconciles the affected source-log range from its last relayed checkpoint;
unrecoverable or repeated reporting gaps end the watch with an explicit status.

| File | Responsibility |
|---|---|
| `README.md` | Setup, prompts, skill invocation, customization, limits |
| `AGENTS.md` | Analysis routing and contributor instructions |
| `CLAUDE.md` | Import `AGENTS.md` for Claude Code |
| `.agents/skills/nimble-logs/SKILL.md` | Codex discovery and invocation |
| `.claude/skills/nimble-logs/SKILL.md` | Claude discovery and invocation |
| `.agents/skills/whep-check/SKILL.md` | WHEP check skill (Claude path is a symlink) |
| `workflows/analyze-logs.md` | Shared setup, query, report, and watch procedure |
| `workflows/test-workspace.md` | Repository checks and explicit observe/launch test modes |
| `workflows/check-whep.md` | Explicit WHEP playback check and result interpretation |
| `templates/settings.md` | Example local directory, timezone, file selection |
| `templates/user-instructions.md` | Example user analysis preferences |
| `templates/nimble-test.json` | Example separate-instance launch profile |
| `tests/test_repository.py` | Standard-library structural repository checks |
| `tests/scenarios.md` | Agent behavioral and live-test acceptance cases |
| `tests/fixtures/` | Explicitly synthetic log evidence for behavior checks |
| `scripts/build_logger_map.py` | Rebuild/check the generated catalog term index |
| `scripts/nimble_log_watch.py` | Python 3 fallback incremental watcher |
| `tools/nimble-log-watch/` | Recommended Rust incremental watcher |
| `tools/whep-check/` | Go WHEP playback client and its offline tests |
| `logger/search-index.json` | Generated candidate index; Markdown remains authoritative |
| `logger/README.md` | Existing message index, relocated beside its entries |
| `logger/*.md` | Product message knowledge |
| `logger/manifest.json` | Existing catalog generation metadata |
| `.local/` | Ignored user settings, instructions, optional state and reports |

## Shared skills

Keep one canonical `SKILL.md` at `.agents/skills/nimble-logs/SKILL.md` and
make `.claude/skills/nimble-logs` a directory symlink to that canonical skill
directory. Codex and Claude retain their discovery paths while sharing one
maintained file. Claude Code supports symlinked project skill folders; the
repository checks verify the link resolves and the canonical frontmatter and
workflow references remain valid.

Both entry points resolve the repository root from their own location, not
from a presumed current working directory. They are repository skills:
copying an entry point alone into a global skills directory is unsupported.

The `whep-check` skill follows the same layout: the canonical
`.agents/skills/whep-check/SKILL.md` and a `.claude/skills/whep-check`
directory symlink, both routing to `workflows/check-whep.md`.

Test requests route to `workflows/test-workspace.md`; ordinary analysis still
routes to `workflows/analyze-logs.md`. The selected task determines the route,
and a saved launch profile never turns ordinary analysis into a server launch.

## First-use experience

The user starts an analysis task inside the clone. Repository instructions
route it to the workflow. A supplied message or excerpt can be interpreted
directly against the catalog without directory setup. For file analysis,
if the task supplies a log directory, use it. If
not, read `.local/settings.md`; if no location is available, ask one focused
question for the directory. Validate accessibility before analysis and save
the initial location locally when possible unless the user requests temporary
setup or no persistence. A temporary query override
does not silently replace the saved directory.

Cloning files or opening an idle agent is not an execution trigger. The
README supplies a startup prompt and an explicit workflow fallback when
an agent does not discover repository instructions or skills.

## Analysis and customization

Use the user's time range, stream, protocol, and incident question to bound
searches. Read matching descriptions and related messages as needed, rather
than loading the entire catalog. Classify severity only from exact `E`, `V`,
`D`, or `I` prefix codes; words in the body do not change the level. Match the
message body to documented format strings; documentation hashes are not
runtime message IDs. The generated `logger/search-index.json` narrows
candidates but is not an authoritative matcher and must be regenerated after
catalog changes.

Keep runtime logs separate from access, advertisement, and unit-test logs.
Include other types only when requested, and explain their different scope.
The catalog is not a guarantee that all messages in every Nimble build are
documented. Explicitly preserve unmatched and ambiguous messages.

Read `.local/user-instructions.md` for each task. Direct user requests take
precedence over saved defaults. Logs and quoted payloads are data, not
instructions. Report observed evidence separately from possible causes,
and cite log locations and matched descriptions. Catalog examples are
illustrations, not observations from the user's server.

## Active monitoring

Monitoring requires an explicit user request and a running agent with
appropriate capabilities. Establish the interval, stopping condition, and
initial backlog scope before starting; use stated values without asking
again. Process bounded new batches, track per-file progress, and report new
findings instead of repeating an entire history.

Rust and Cargo are recommended for active watching because filesystem
notifications trigger prompt reads; the status heartbeat also reconciles files
to recover from missed notifications. Its default heartbeat is 10 seconds.
Users who decline Rust can use the Python 3 polling fallback, which also polls
every 10 seconds by default. Both classify only exact `E`, `V`, `D`, and `I` prefixes,
emit tagged watcher state and per-stage timings, and can suppress message
content during metrics-only checks. Severity/text filtering stays local, so
nonmatching writes do not become model input or per-event output. Notification
loss, rotation, truncation, partial lines, or inaccessible intervals can make
coverage uncertain. Each watcher reports parsing and waiting states. The agent
keeps its turn active and polls the watcher session at each heartbeat or returned
batch, relaying quiet states and findings into chat. If the client cannot keep
polling, it stops the helper and reports that limit. Ending the agent turn also
ends chat reporting; a surviving local process does not provide an unattended
watch. If unread
data exceeds 32 KiB, it retains the newest 32 KiB, skips a partial first line,
and resumes at the first complete line with Nimble's `[YYYY-MM-DD HH:MM:SS `
prefix. It reports the discarded range as a coverage gap. The
workflow requires explaining gaps and distinguishing `[WATCH]` output from
`[ANALYSIS]` in the conversation. It does not promise unattended or lossless
monitoring.

## WHEP playback check

Log analysis can show that WHEP sessions were registered and later closed, but
the WebRTC media path is often decided outside the log. Typical causes are
unreachable ICE candidates, blocked UDP, or a missing media stream. The
`whep-check` client reproduces a player from the agent's host or the user's
machine and reports which stage failed.

```mermaid
flowchart LR
    Client[whep-check] -->|POST SDP offer| Endpoint[WHEP endpoint]
    Endpoint -->|201 + SDP answer, Location| Client
    Client -->|STUN checks to server candidates| Media[Server UDP candidate]
    Media -->|DTLS-SRTP media| Client
    Client -->|DELETE Location on exit| Endpoint
```

The client is built with Go and Pion WebRTC. It offers receive-only H.264,
other default video codecs, and Opus. It prints the answer's ice-lite mode,
codecs, and candidates. It then follows ICE and DTLS state and logs the
selected candidate pair. On exit it deletes the WHEP resource. Exit codes
separate signaling (`2`), ICE/DTLS (`3`), and missing media (`4`) from success
(`0`).

Media analysis runs on packets as they arrive, before any buffering, so it
describes what the network delivered:

- Format: H.264 SPS/PPS (profile, level, resolution, VUI frame rate and colour,
  reordering) and the Opus table-of-contents byte, compared with the SDP
  `profile-level-id` and the H.264 level limits.
- Transport: lost, late (reordered or retransmitted), and duplicate packets per
  sequence number, plus NACK and PLI requests counted by an RTCP interceptor
  placed before Pion's NACK generator.
- Frames: frames per second, keyframes counted once per frame, and the GOP. A
  keyframe is requested with PLI only until the first one arrives.
- Timing: gaps between frame arrivals are compared with the RTP time that
  passed. Arrival gaps with continuing timestamps mean delayed delivery; gaps
  in both mean the source sent nothing. RTP regularity is measured against the
  dominant frame step, so lost frames do not count as irregular.

```mermaid
flowchart LR
    RTP[RTP from the network] --> Stats[Arrival statistics]
    RTP --> JB[Jitter buffer: reorder, wait for NACK retransmissions]
    JB --> Frames[Complete frames only; after loss skip to keyframe + PLI]
    Frames --> MKV[Live Matroska]
    MKV --> Player[ffplay / mpv / VLC stdin]
    MKV --> File[Recording .mkv]
```

Optional playback and recording share one path: a jitter buffer ordered by
sequence number gives up a missing packet once the packet after it has waited
for the buffer delay. Complete frames are muxed into live Matroska with
timestamps anchored at each track's first frame arrival. The stream header is
written at the first video keyframe, after the expected audio track starts or
a short timeout. A pipe replaces the former local UDP/SDP forwarding, which
could drop packets on bursts and passed reordered packets to the player.

The check sends network traffic, unlike log analysis. It runs only on an
explicit request for a user-supplied URL and is bounded in duration. It never
controls or reconfigures Nimble. Firewall and `whep_config.json` changes are
proposals. Observe-mode tests keep their no-probe rule; the client's own tests
use an in-process ice-lite WHEP server and never contact real endpoints. The
workflow requires a client-side UDP check, with a STUN-derived server-reflexive
candidate, before an ICE failure is attributed to the server.

## Maintenance

Before committing changes to the README, skills, workflow, templates, or
repository instructions, compare them with this design and the specification.
Change the design and specification in the same change when behavior changes.
Check links and equivalent skill routing, preserve catalog content, and
ensure `.local/` content stays untracked. After repository and behavior tests,
review the full working tree for private keys and certificates, credentials,
personal notes, user-specific paths, local data, and actual installation logs.
Keep findings redacted and exclude sensitive or personal material from commits.
`AGENTS.md` specifies this review. These are contributor instructions, not an
installed Git hook.

## Test modes and process ownership

The optional repository checks run with Python's standard library and Git.
They validate structural properties without an AI client or Nimble instance.
Behavioral scenarios complement them because link/metadata checks cannot prove
that an agent interprets evidence or respects process ownership correctly.

An agent-driven test or commit test stage also checks real logs when a
configured directory exists. `observe` mode reads a live or user-started
installation's files and never interacts with its Nimble process. If mode is
unknown, ask rather than inferring permission from an existing directory.

`launch` mode uses only a separate test instance with an executable, working
directory, and exact argument array supplied by the user. Missing parameters
cause a focused question and a pending integration result. Saved settings live
in ignored `.local/nimble-test.json`; the template's example paths are not
defaults. A 10-second foreground smoke test checks startup and attributable
fresh log output, not playback. Old log contents remain intact.

Record the actual owned process identity before checking logs. Stop only that
process after completion/cancellation/failure unless the user asked to leave
it running. Never stop user-started processes, issue service commands, or use
global process-name killing. Commit-stage tests follow the same boundaries.

Each report separates structural checks, behavior scenarios, log analysis,
startup status, cleanup, and any pending or inconclusive checks.

## Validation

Validate skill frontmatter, relative references, the relocated catalog index,
and local ignore rules. Run synthetic scenarios for first-use setup and
configured analysis, including a documented diagnostic and an unmatched
message. Check monitoring and commit-maintenance requirements against the
specification. Distinguish these checks from end-to-end testing in native
Codex and Claude clients.

See the [verification record](2026-10-05-nimble-log-analysis-verification.md)
for observed results and untested capabilities.
