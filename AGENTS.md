# Repository instructions

This repository provides Nimble Streamer log descriptions and a Markdown
analysis workflow for users running their own local AI agents.

## Log-analysis tasks

When asked to analyze Nimble logs, explain runtime messages, investigate an
incident, or watch new events, read and follow
[workflows/analyze-logs.md](workflows/analyze-logs.md). That document defines
directory setup, local preferences, matching, evidence, and watch limits. For
an active watch, keep the agent turn polling the watcher tool session and relay
its state and findings into chat; tool output alone is not a user-facing report.

For file analysis, ask for the log directory only if neither the task nor
`.local/settings.md` provides it. Interpret supplied messages directly without
requiring directory setup. Read `.local/user-instructions.md` on each analysis
task. Current user requests override saved defaults. Keep source logs read-only
and user-specific settings, checkpoints, and reports under ignored `.local/`
unless the user requests a different output location.

## Feature skills

Other checks have their own skill, workflow, and tools. When a request matches
a row, read and follow that workflow. Feature checks run only on an explicit
request, never during log analysis or workspace tests, and they propose server,
network, or firewall changes instead of applying them.

| Request | Workflow | Boundary |
|---|---|---|
| Check WebRTC WHEP playback from an endpoint URL, or explain why a WHEP player does not start | [workflows/check-whep.md](workflows/check-whep.md) | Opens a real viewer session; only for a user-supplied URL |

For repository editing or documentation questions, follow the contributor
instructions below; do not trigger log setup or ask for a log directory.

## Test requests

For workspace tests and required commit-stage tests, read
[workflows/test-workspace.md](workflows/test-workspace.md). Run the Python
repository checks and relevant [behavior cases](tests/scenarios.md). When a
supplied/configured log directory exists, also check its logs in the chosen mode.
If required test mode or launch details are missing, ask for them while finishing
independent checks; report the log integration test as pending.

Ordinary log analysis and tests of a live/user-started installation are read-only.
Never start, stop, restart, signal, attach to, or probe Nimble in those cases.
Only launch a separate test instance with user-supplied executable, working
directory, and arguments. Save explicit test settings in ignored
`.local/nimble-test.json`. Stop only the process that this test actually started,
unless the user asked to leave that separate instance running.

## Contributor instructions

The maintained behavior is described in:

- [Design](docs/superpowers/2026-10-05-nimble-log-analysis-design.md)
- [Specification](docs/superpowers/2026-10-05-nimble-log-analysis-spec.md)
- [Implementation plan](docs/superpowers/plans/2026-10-05-nimble-log-analysis.md)

Read the design and specification before changing `README.md`, any skill,
the workflows, templates, or repository instructions. Keep the analysis procedure
in `workflows/analyze-logs.md`. Every skill's canonical file is
`.agents/skills/<name>/SKILL.md`; `.claude/skills/<name>` must be a directory
symlink to that skill directory so both clients discover one maintained file.
`CLAUDE.md` imports this file.

A feature skill keeps its material in its own files so that features do not
edit the same lines: the procedure in `workflows/<workflow>.md`, a row in the
feature table above, tools in `tools/<tool>/` (with a `README.md` that lists
their tests and a local `.gitignore` for build output), the design, contract,
and acceptance checks in `docs/superpowers/features/<name>.md`, behavior cases
in `tests/scenarios/<name>.md`, and skill-specific checks in
`tests/test_feature_<name>.py` (hyphens written as underscores). Shared files
carry only rules that apply to every skill.

Before committing a change to those files:

1. Compare the change against the specification's scope and acceptance checks.
   If behavior changes, update the design/specification and affected usage docs,
   skills, workflow, and templates in the same change.
2. Confirm README prompts and documented capabilities follow the workflow.
   Preserve the first-use directory question, persistent local preferences,
   evidence citations, unknown-message handling, and active-session watch limits.
3. Verify canonical skill frontmatter, the Claude directory symlink target, and
   links to the shared procedure. Check relative Markdown links and the catalog index in
   `logger/README.md`. End users must not need Superpowers or a Nimble source clone.
4. Run `git diff --check` and inspect the proposed staged diff. Ensure `.local/`
   and actual user logs are absent from it. `git check-ignore .local/settings.md
   .local/user-instructions.md .local/state.md .local/reports/example.md` should
   show every example path as ignored.
5. Run `python3 -m unittest discover -s tests -v` and relevant synthetic cases
   from `tests/scenarios.md` and `tests/scenarios/`. If a configured log directory exists, follow the
   shared test workflow to check real logs in `observe` or explicitly configured
   `launch` mode. Ask for missing test mode/launch details instead of skipping
   this stage silently. A commit-stage test must never control a live installation.
   Report pending, failed, or inconclusive integration and any native-client or
   monitoring checks not run; structural success is not an overall test pass.
6. Run `python3 scripts/build_logger_map.py --check` after catalog changes and
   `cargo test --locked --manifest-path tools/nimble-log-watch/Cargo.toml`
   when changing the Rust watcher or its parser. When changing a feature tool,
   run the tests listed in `tools/<tool>/README.md`; they must use synthetic
   data and must not contact real endpoints or control a real Nimble.
   Keep `logger/search-index.json` generated from Markdown; do not hand-edit it.
   Watcher changes must also exercise the Python fallback, Rust change-triggered
   behavior, tagged status, and per-stage timings on synthetic logs. Real-log
   performance checks must use `--metrics-only` and report coverage without
   exposing message contents.
7. After all repository, behavior, map, and tool tests finish, perform a full
   working-tree security review before committing. Inspect staged and unstaged
   changes, untracked files, and hidden or ignored repository-local files that
   could be staged. Look for private key or certificate files (including
   `.pem`, `.key`, `.p12`, and `.pfx`), passwords, tokens, API keys,
   credential-bearing connection strings, personal notes, user-specific paths,
   actual Nimble logs, and `.local/` data. Treat file extensions and search
   matches as prompts for review; distinguish public certificates and
   explicitly synthetic fixtures from private or user-specific material. Use a
   local secret scanner when available. Inspect findings without printing
   secret values in chat, reports, or command output; record only paths and
   redacted categories. Exclude sensitive or personal material from the commit
   and keep it local. Do not delete user files automatically.

Preserve existing catalog entries and `manifest.json` unless the task explicitly
changes the catalog. Manifest checksums describe a generated snapshot and may
already differ from edited documentation; do not rewrite them to conceal drift.
Message-index links belong relative to `logger/README.md`.
Only explicitly synthetic fixture logs under `tests/fixtures/` belong in source
control; actual installation logs and launch details must stay local.

These instructions guide contributor and AI-agent review. They do not install
a Git hook or enforce commits automatically. Complete requested edits and checks
without committing or publishing unless the user asks for that action.
