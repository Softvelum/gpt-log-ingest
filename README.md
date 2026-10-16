# Analyze Nimble Streamer logs with your AI agent

Clone this repository and use your own local AI agent to investigate
[Nimble Streamer](https://softvelum.com/nimble/) logs. The repository supplies
message descriptions, a shared analysis workflow, and skills for Codex and
Claude Code. Your agent reads the logs from the directory you provide; Nimble
needs no changes, and you do not need its source code or Superpowers.

## Quick start

```bash
git clone https://github.com/Softvelum/gpt-log-ingest.git
cd gpt-log-ingest
codex
# Or launch Claude Code instead:
# claude
```

Start an analysis task:

```text
Help me analyze Nimble logs.
```

The repository instructions tell the agent to ask for your log directory when
none is supplied or saved. For example, answer `~/Development/nimble/log` or
`/var/log/nimble`. The agent checks the location and saves it in ignored
`.local/settings.md` when possible. Later questions reuse that location.

You can also provide the directory and question together:

```text
Analyze Nimble logs in ~/Development/nimble/log. Explain the errors from the
last hour and show the evidence for any failures affecting live/camera1.
```

The agent must have permission to read that directory. Cloning the repository
or opening an idle agent does not itself start analysis. If the client does not
load the repository instructions, use this explicit fallback:

```text
Read workflows/analyze-logs.md in this repository and follow it to analyze
my Nimble logs. Ask me for the log directory if it is not already configured.
```

## Use the skills

Launch the agent from this clone so it can discover the repository skill.

| Agent | Explicit invocation | Skill location |
|---|---|---|
| Codex | `$nimble-logs` | [.agents/skills/nimble-logs/SKILL.md](.agents/skills/nimble-logs/SKILL.md) |
| Claude Code | `/nimble-logs` | [.claude/skills/nimble-logs/SKILL.md](.claude/skills/nimble-logs/SKILL.md) |
| Codex | `$whep-check` | [.agents/skills/whep-check/SKILL.md](.agents/skills/whep-check/SKILL.md) |
| Claude Code | `/whep-check` | [.claude/skills/whep-check/SKILL.md](.claude/skills/whep-check/SKILL.md) |

For example, in Codex:

```text
$nimble-logs Explain why live/camera1 disconnected around 14:30 today.
```

In Claude Code:

```text
/nimble-logs Explain why live/camera1 disconnected around 14:30 today.
```

Clients may also select the skill from a natural-language request. The canonical
skill lives under `.agents/skills/nimble-logs/`; Claude's project skill directory
links there, so both clients use the same instructions and
[analysis workflow](workflows/analyze-logs.md). Keep the skill with this
repository: copying its entry file alone into a global skills directory will
break its references.

See the official [Codex skill documentation](https://learn.chatgpt.com/docs/build-skills)
and [Claude skill documentation](https://code.claude.com/docs/en/skills) for
client discovery and invocation behavior.

## Ask questions about existing logs

```text
Group the errors from the last hour. Which ones affected playback?

Investigate SRT disconnects for live/camera1 between 14:20 and 14:40 today.

Compare today's failures with yesterday's and identify new patterns.

Explain this message using the catalog and tell me what to check next:
failed to detect country code for '8.8.8.8'
```

Explaining a pasted message uses the catalog directly and does not require a
log directory. Additional log files are needed to establish the incident's
surrounding events or actual server impact.

For directory analysis, the agent normally selects `nimble.log` and
`nimble-*.log`, including relevant rotated files. Access, advertisement, and
unit-test logs are excluded by default; request them explicitly when needed.
If timestamps lack an offset, the agent uses the saved log timezone or reports
log-local time. Cross-timezone comparisons may require a timezone question.

Reports should answer your question with timestamps, source files and line
references, matched descriptions, measured counts, and suggested next checks.
They should distinguish observed impact from likely causes and explain
unmatched messages or incomplete coverage. The agent reads source logs without
modifying them; remediation suggestions are proposals for you to review.

## Save your own instructions

Ask the agent to persist preferences, for example:

```text
Save these preferences in .local/user-instructions.md: report in Russian,
prioritize SRT inputs, group repeated messages, and redact tokens in excerpts.
```

The workflow reads that file on each analysis task. Your current request
overrides saved defaults. A one-off query against another directory does not
change the saved default unless you ask it to.

For manual setup, copy the examples and edit them for your machine:

```bash
mkdir -p .local
cp templates/settings.md .local/settings.md
cp templates/user-instructions.md .local/user-instructions.md
```

The [settings template](templates/settings.md) holds the directory, log timezone,
and file patterns. The [user-instructions template](templates/user-instructions.md)
holds reporting preferences and incident context. All `.local/` content is
ignored by Git. Saved reports, when requested, default to `.local/reports/`.

## Watch new logs during a session

```text
Watch my Nimble logs for 30 minutes, checking every 10 seconds. Start with new
lines only. Report new SRT failures and changes affecting live/camera1.
```

Watching requires an explicit request, a running agent, and an agreed interval,
duration or stop condition, and backlog scope. Rust and Cargo are recommended
for filesystem-event processing, local grouping, and candidate ranking before
log batches are returned to the agent. The helper requires Rust 1.70 or later.
Install the Rust toolchain from https://rustup.rs/, then build it once:

```bash
cargo build --release --locked --manifest-path tools/nimble-log-watch/Cargo.toml
tools/nimble-log-watch/target/release/nimble-log-watch \
  --dir ~/Development/nimble/log --levels E --status-interval-seconds 10
```

Rust processes filesystem notifications as they arrive and performs a
reconciliation scan on each status heartbeat. The heartbeat defaults to 10
seconds; it is not the normal log-processing interval. Output uses `[WATCH]`
state, batch, and performance labels, with color in terminals. Use
`--levels E,V,D,I` for all four codes, `--duration-seconds 1800` for a 30-minute
run, or `--metrics-only` to benchmark processing without showing messages. The
default starts at each current file end and watches until interrupted. If you
decline installing Rust, the Python 3 fallback polls at its interval:

```bash
python3 scripts/nimble_log_watch.py \
  --dir ~/Development/nimble/log --levels E --interval-seconds 10
```

Both tools recognize severity only from Nimble's exact `E`, `V`, `D`, and `I`
prefix codes; words in a message body do not determine severity. They group
repeated records and use the generated catalog index to suggest Markdown
descriptions. They report read, parse, grouping, catalog, output, and total
timings per batch. The agent must still verify each candidate description.
Both apply requested level filters locally and stay quiet for batches without
matching records; raw nonmatching lines are not sent to the agent. Periodic
status and coverage gaps remain visible.
Both watchers emit parsing and waiting state. If unread data exceeds 32 KiB,
they retain the newest 32 KiB, skip a partial line, and resume at the next
complete Nimble record beginning with `[YYYY-MM-DD HH:MM:SS `. They report
skipped bytes as a coverage gap.
Filesystem notification limits, rotation, or inaccessible files can cause
reported coverage gaps. Monitoring ends with your cancellation, the stop
condition, or the agent session; this does not provide unattended scheduling
or guaranteed lossless delivery.

The watcher writes to its running tool session; it cannot push updates directly
into chat. During a watch, the agent must keep its turn active, poll that
session at each heartbeat or returned batch with short waits, and relay status
and findings into the conversation before lengthy catalog work. Progress
updates group identical messages while preserving distinct errors, counts,
short excerpts, reader-supported source positions, match confidence, and the
last checkpoint. Byte offsets are used when available, never invented; excerpt
redaction follows the user's instructions. Quiet heartbeats are reported too.
If tool output is clipped, the agent must label the report gap and partial
counts, reconcile the bounded source-log range, and stop if it cannot recover
the missing output. If the client cannot continue polling, the agent should
stop the helper and explain the limit. A local watcher process left running
after the agent turn ends will not continue chat reporting.

## Feature skills

Besides log analysis, the repository has skills for specific checks. Each one
has its own workflow and tools; its README explains usage, options, and tests.
The agent runs these checks only when you ask for them and proposes, rather
than applies, server and network changes.

| Skill | Use it for | Details |
|---|---|---|
| `whep-check` | Check WebRTC WHEP playback from an endpoint URL and find the failing stage | [tools/whep-check](tools/whep-check/README.md) |

## How the instructions fit together

| Resource | Used for |
|---|---|
| [AGENTS.md](AGENTS.md) | Routes analysis tasks and defines contributor checks |
| [CLAUDE.md](CLAUDE.md) | Imports the shared repository instructions for Claude |
| The two `nimble-logs` skills | Convenient discovery and explicit invocation |
| [Shared workflow](workflows/analyze-logs.md) | Setup, queries, evidence, and active watching |
| [Test workflow](workflows/test-workspace.md) | Repository checks and read-only or separate-instance log tests |
| Feature workflows in `workflows/` | Explicit feature checks, listed under [Feature skills](#feature-skills) |
| `.local/settings.md` | Your saved log location and defaults |
| `.local/user-instructions.md` | Your analysis and reporting preferences |
| `.local/nimble-test.json` | Explicit test mode and optional launch parameters |
| [Message catalog](logger/README.md) | Patterns, triggers, explanations, and checks |

The catalog currently contains 3,410 fixed-message descriptions. Documentation
IDs identify entries; they are not runtime message IDs. Some runtime or
build-specific messages may have no matching description. The existing
[manifest](logger/manifest.json) describes the generated catalog snapshot; it is not
an analysis engine or a guarantee that edited files retain snapshot checksums.

## Run tests

For automated repository checks, run from this clone:

```bash
python3 -m unittest discover -s tests -v
```

These standard-library checks verify skill metadata/equivalence, workflow and
documentation links, catalog-index coverage, generated-map freshness, ignore
rules, and the launch template. They never start Nimble or call an AI model.

To have your agent run the checks and analyze installation logs, ask:

```text
Run the workspace tests and check logs in ~/Development/nimble/log.
This is a live installation. Read its logs only; do not interact with Nimble.
```

For a separate test instance:

```text
Run the workspace tests, launch a separate Nimble test instance, and check
its new logs. Ask me for the executable, working directory, and launch arguments
if those details are not already saved.
```

Both skills route test requests to [the shared test workflow](workflows/test-workspace.md).
The agent first establishes `observe` or `launch` mode. Live installations and
instances you started are read-only: the agent never starts, stops, restarts,
signals, attaches to, or probes them. It may stop only a separate instance that
it started for the test, unless you asked it to leave that instance running.

Save read-only mode in `.local/nimble-test.json` as `{"mode": "observe"}`.
For separate launch tests, the agent saves your supplied executable, working
directory, and argument array there; [the launch template](templates/nimble-test.json)
shows the format. Replace example paths with your own details. The default
managed test is a 10-second foreground smoke test, without Nimble's `-d` flag.
It checks process startup and attributable fresh log output, then interprets
the output using the catalog. It does not prove playback or streaming health.

The [behavior scenarios](tests/scenarios.md) cover setup, supplied messages,
evidence, missing launch details, live-installation boundaries, and owned-process
cleanup. An agent should report which scenarios were actually exercised.
Missing launch details or an unavailable live check stay pending; passing the
Python checks alone does not mean all tests passed.

When catalog descriptions change, regenerate the Rust/Python candidate map and
check that it matches the source files:

```bash
python3 scripts/build_logger_map.py
python3 scripts/build_logger_map.py --check
cargo test --locked --manifest-path tools/nimble-log-watch/Cargo.toml
```

Each feature tool lists its own tests in its README (see
[Feature skills](#feature-skills)). They use synthetic data and never contact
real endpoints or control a real Nimble.

## Contributing

Read the [design](docs/superpowers/2026-10-05-nimble-log-analysis-design.md),
[specification](docs/superpowers/2026-10-05-nimble-log-analysis-spec.md), and
[implementation plan](docs/superpowers/plans/2026-10-05-nimble-log-analysis.md)
before changing public usage or analysis behavior.

Before committing changes to the README, skills, workflow, templates, or
repository instructions, follow the consistency checks in [AGENTS.md](AGENTS.md).
Update the design/specification alongside behavior changes, keep the skill
Claude's skill-directory symlink aimed at the canonical `.agents` skill, verify
links, and keep user-local files out of commits.
These are contributor instructions; this repository does not install a Git hook.
The commit test stage includes the Python checks, relevant agent scenarios,
and a log check when a configured log directory exists. It follows the same
read-only/live and separate-instance rules and asks for missing test settings.
After all required repository, map, Rust, and behavior checks, review the full
working tree for private keys or certificates, credentials, personal notes,
user-specific paths, and local Nimble data before committing. Keep findings
redacted and exclude sensitive or personal material from the commit; see
[the test workflow](workflows/test-workspace.md).
