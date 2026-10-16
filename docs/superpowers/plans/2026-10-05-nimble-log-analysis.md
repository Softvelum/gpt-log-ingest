# Nimble Log Analysis Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans when available to
> implement this plan inline. The user requested the artifacts in this session;
> no production server changes or external publication are part of this plan.

**Goal:** Make this clone usable for conversational analysis of local Nimble logs.

**Architecture:** Repository instructions and two equivalent skill entry points
route to one Markdown workflow. Rust is recommended for incremental watching;
Python 3 is the fallback. Both use a generated JSON candidate index while
existing Markdown descriptions remain the knowledge source.

**Tech Stack:** Markdown, Rust/Cargo, Python 3, standard Agent Skills frontmatter.

**Spec:** [Nimble log analysis specification](../2026-10-05-nimble-log-analysis-spec.md)
and [design](../2026-10-05-nimble-log-analysis-design.md).

## Global constraints

- Maintain one shared analysis procedure and portable Codex/Claude entry points.
- Read source logs without modifying them.
- Keep existing message descriptions and `manifest.json` unchanged in this task.
- Ignore `.local/` in Git, including settings, user instructions, state, and reports.
- Watch helpers remain local readers; no autonomous scheduler, real-time
  guarantee, or unattended execution is included.

## Review focus

- Missing directory: ask for it once, without searching unrelated directories.
- Supplied excerpt: explain it through the catalog without directory setup.
- Saved preferences: read them on subsequent queries and preserve their contents.
- Mixed files: runtime analysis excludes access, advertisement, and test logs.
- Diagnostic/unknown events: explain documented impact and retain uncertainty.
- Watch lifecycle: explain coverage gaps and stop without claiming background work.

## Task 1: Shared procedure and local customization

**Files:** `workflows/analyze-logs.md`, `templates/settings.md`,
`templates/user-instructions.md`, `.gitignore`.

**Interfaces:** Inputs are the user's task, readable logs, local Markdown settings,
and `logger/` descriptions. Output is an evidence-based report and optional
local persistence; there is no parser or watcher API.

- [x] Run synthetic baseline queries before introducing analysis instructions.
- [x] Write setup, search, reporting, and active-watch guidance against the spec.
- [x] Provide editable templates and ignore `.local/`.
- [x] Check directory precedence, preference loading, mixed-file scope, and watch
  limitations against the specification.

## Task 2: Skills, repository instructions, and usage documentation

**Files:** `.agents/skills/nimble-logs/SKILL.md`,
`.claude/skills/nimble-logs/SKILL.md`, `AGENTS.md`, `CLAUDE.md`, `README.md`,
`logger/README.md`.

**Interfaces:** Both skills read `../../../workflows/analyze-logs.md` relative to
their own file. Claude imports `AGENTS.md`; repository instructions route only
log-analysis requests and govern contributor changes.

- [x] Add the small Codex skill and validate its metadata and routing.
- [x] Add the equivalent Claude entry point and validate both.
- [x] Preserve the message index under `logger/README.md`; rewrite root usage docs.
- [x] Add before-commit design/spec consistency instructions to `AGENTS.md`.

## Task 3: Verify the deliverables

**Interfaces:** Structural checks examine Markdown paths, equivalent skill files,
index links, ignore rules, and diffs. Isolated agent scenarios read synthetic logs.

- [x] Run the skill validator for both discovery locations.
- [x] Verify local links, all catalog-index entries, skill equivalence, and ignores.
- [x] Run configured and first-use scenarios using the new instructions, including
  a documented diagnostic, an unmatched event, and a pasted-message explanation.
- [x] Review the watch lifecycle and contributor checks against the spec.
- [x] Record verification outcomes and remaining native-client/live-watch limits.

## Task 4: Test workflow and installation ownership extension

**Files:** `tests/test_repository.py`, `tests/scenarios.md`,
`tests/fixtures/nimble-synthetic.log`, `workflows/test-workspace.md`,
`templates/nimble-test.json`, both skills, README, AGENTS, analysis workflow,
settings template, design/specification, and verification record.

**Interfaces:** Repository checks run with Python's standard library. The shared
test workflow reads `.local/settings.md` and optional `.local/nimble-test.json`.
`observe` only reads logs. `launch` requires a user-supplied executable, working
directory, and argument array, records process ownership, and inspects fresh logs.

- [x] Write repository checks first and confirm missing test routes/template fail.
- [x] Add the shared test workflow, local launch template, and behavior fixtures.
- [x] Route test requests from both equivalent skill entry points and AGENTS.
- [x] Document read-only live/user-started installations and owned-process cleanup.
- [x] Align README and design/specification with the new test procedure.
- [x] Run the repository suite and isolated mode/setup behavior checks.
- [x] Run the user-selected real log check; require launch details only for a
  separate managed instance, and retain missing integration as pending.
- [x] Record actual verification outcomes, including untested launch behavior.

## Task 5: Fast exact-level watching and catalog candidate index

**Files:** Rust watcher, Python fallback, map generator and generated map,
analysis workflow, skills, README, templates, synthetic fixture, tests, and
design/specification.

**Interfaces:** Both tools read appended runtime logs from end-of-file by
default, classify only exact `E`, `V`, `D`, and `I` codes, group repeats, and
rank descriptions using the generated `logger/search-index.json`. Markdown
remains authoritative. Rust/Cargo is recommended; Python 3 is the fallback.

- [x] Add Rust parser/grouping tests for exact codes and catalog candidates.
- [x] Add Python standard-library map generation and freshness check.
- [x] Add bounded incremental readers with incomplete-line retention and gap reporting.
- [x] Document build/run commands and Python fallback across entry points.
- [x] Verify both tools against synthetic E/V/D/I records and repeat grouping.
- [x] Run map, Rust, Python repository, and diff checks.

## Follow-up: responsive watcher output and instrumentation

Rust watching is event-driven with a heartbeat reconciliation scan; Python is
the polling fallback. Both emit tagged state and phase timings, with a
metrics-only option for content-suppressed performance checks.

- [x] Verify prompt Rust append handling, status transitions, and metrics-only.
- [x] Verify Python polling, exact level classification, timings, and metrics-only.
- [x] Document the processing and heartbeat distinction, status labels, colors,
  pause visibility, and timing boundaries.
- [x] Run a bounded content-suppressed check against the configured Nimble logs.

## Follow-up: one canonical skill file

Use `.agents/skills/nimble-logs/SKILL.md` as the canonical file and make
`.claude/skills/nimble-logs` a symlink to that directory. Keep both invocation
paths documented and test the link target plus shared workflow references.

- [x] Replace the duplicate Claude skill directory with a symlink.
- [x] Update repository instructions, usage docs, and spec acceptance checks.
- [x] Verify one skill file serves both client discovery paths.

## Follow-up: keep active watcher output visible in chat

Make active polling a required part of the watch contract. The watcher writes
to its tool session and does not push directly into chat, so the agent must keep
its turn active, poll in short slices, and promptly relay each heartbeat/batch.
Group identical records without dropping distinct evidence. If tool output is
clipped, reconcile the bounded source range from the last relayed checkpoint;
stop with a separate reporting-gap status if recovery is unreliable. Stop the
helper if continued polling is unavailable.

- [x] Add a failing repository assertion for active polling, chat relay, and the
  lack of push reporting beyond the active agent turn.
- [x] Update the workflow, canonical skill, README, design, specification, and
  behavior scenario with prompt grouped relays and clipped-output recovery.
- [x] Run the full repository suite, scenario review, link checks, and diff review.

## Follow-up: feature skill layout

Move feature-specific material out of shared files so that features developed
in parallel do not edit the same lines.

- [x] Route feature skills from one table in `AGENTS.md` and `README.md`; state
  skill, tool, and test rules once for every skill.
- [x] Move each feature's design, contract, acceptance checks, and plan to
  `docs/superpowers/features/<name>.md`, its behavior cases to
  `tests/scenarios/<name>.md`, its usage and tests to `tools/<tool>/README.md`,
  and its build ignores to `tools/<tool>/.gitignore`.
- [x] Discover skills in repository checks from `.agents/skills/`; move
  skill-specific assertions to `tests/test_feature_<name>.py`.
