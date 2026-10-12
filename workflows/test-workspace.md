# Test the Nimble log workspace

Use this workflow when the user asks to run workspace/skill tests, or when
repository instructions require tests before a commit. Resolve repository
paths from the parent of `workflows/`. Ordinary log analysis uses
[analyze-logs.md](analyze-logs.md) and never launches or controls Nimble.

## 1. Run the repository checks

From the repository root, run:

```bash
python3 -m unittest discover -s tests -v
git diff --check
```

The standard-library suite checks canonical skill discovery metadata, the
Claude directory symlink, links to both shared workflows, public document references,
catalog-index coverage, local ignore rules, and the launch-profile template.
It never starts Nimble or invokes an AI client. Passing it proves those
structural properties, not that an agent made a correct diagnosis.

When the change touches `tools/whep-check`, also run `go vet ./...` and
`go test ./...` in that directory. These tests use an in-process WHEP server.
Never run the WHEP client against a real endpoint as part of workspace or
commit tests; [WHEP checks](check-whep.md) are separate, explicit requests.

Exercise relevant cases in [the behavioral scenarios](../tests/scenarios.md)
when changing analysis or test behavior. Run them in a temporary workspace,
using synthetic fixtures and local preferences rather than real installation
configuration. Native client discovery requires a separate client check.

## 2. Resolve logs and test mode

Read `.local/settings.md`, `.local/user-instructions.md`, and
`.local/nimble-test.json` when present. Current user instructions override
saved defaults. Resolve and check the log directory as in the analysis
workflow; a readable directory alone does not authorize starting a server.

For an agent-driven test request, check real logs when a directory is supplied
or configured and exists. The same log check is required during the commit
test stage when a configured directory exists. If no directory is available
and a live check is requested, ask for it while allowing structural checks
to finish. Report any missing live check as pending, not as a passed test.

Choose one of these explicit modes:

| Mode | Process behavior | Log check |
|---|---|---|
| `observe` | Read files only; never interact with Nimble | Inspect existing logs and optionally new records during a bounded observation |
| `launch` | Launch a separate, user-configured foreground test instance | Inspect the fresh output attributable to that launch |

If the user says the installation is live, existing, or that they will start
Nimble themselves, use `observe`. **Do not start, stop, restart, signal,
attach a debugger to, or send test/probe traffic to a live installation.**
Do not call service-management commands or terminate a process merely because
its name, PID file, ports, or log directory looks like Nimble.

For a separate instance, use `launch` only when the user requested it or
saved that test mode explicitly. If mode is unknown, ask whether to inspect
an existing installation read-only or launch a separate test instance. Do not
infer the answer from a directory's existence. Save a chosen test mode locally
unless the user requests a temporary override.

## 3. Ask for and retain launch details

Skip this section in `observe` mode. A live installation never requires a
launch command merely to inspect its logs.

For `launch`, resolve the executable, working directory, and exact arguments
from the current request or `.local/nimble-test.json`. If any are missing, ask:

> What Nimble executable and working directory should I use, and what exact
> launch arguments are needed, including configuration and rules files?

An empty argument list is valid if explicitly supplied. Do not invent a
binary, configuration, port, or flags from this repository's examples. Store
the supplied launch details locally using
[templates/nimble-test.json](../templates/nimble-test.json). Its `/path/to/...`
values are examples, not usable defaults. A read-only profile can simply be:

```json
{"mode": "observe"}
```

The launch template separates the executable, working directory, and argument
array. Expand and resolve the executable and working-directory paths. Relative
paths inside arguments are interpreted by Nimble in that working directory;
do not rewrite ordinary argument values or flags. Verify the executable and
working directory exist. Keep the argument array intact; use argument-vector
execution where available, or quote each
argument correctly for the agent's command tool. Never evaluate saved strings
as shell code. Redact sensitive argument values in reports.

Use the log directory already supplied/saved. Check that the user's launch
configuration directs output there; resolve conflicting log destinations
before starting. Do not add flags or rewrite configuration to force a match.
Reject Nimble's `-d` daemon option for a managed test: the agent must retain
ownership of the foreground process. Ask for foreground test arguments instead.

Use the supplied bounded duration, otherwise the launch template's 10-second
smoke-test duration. Stop an agent-owned test instance afterward by default.
If the user explicitly requests leaving it running or saved `stop_after_test`
as `false`, retain its process identifier and report that it is still running;
do not claim cleanup.

## 4. Collect evidence and run the selected check

Before launch or observation, record the start time and the selected runtime
files' identities and sizes, plus the final complete line where available.
Exclude access, advertisement, and test logs unless requested. Keep existing
files intact: do not truncate, clear, rotate, rename, or delete them.

### Read-only installation

Inspect a bounded, stated scope of the existing logs through the analysis
workflow. If the user requests a timed observation, read appended records
without interacting with the process. No new records during an idle interval
is a coverage result, not evidence that Nimble failed. Do not claim to have
tested startup, connectivity, or process health through file reading alone.

### Separate managed test instance

Launch exactly the supplied executable/arguments in the supplied working
directory. Use a process tool or subprocess that provides the actual owned
process ID/session and permits bounded waits. Capture stdout/stderr under
`.local/reports/` if needed; they are startup evidence, not substitutes for
checking the configured log files.

Record that process ID/session immediately. If startup fails or the process
exits early, retain its actual exit status and startup output. Do not repeatedly
restart it, kill an existing instance, change server configuration, or switch
ports to make the smoke test pass. Explain the failure.

Wait for the bounded test interval, allowing interruption. Then inspect new
log output, including available files created during the run. Use start offsets,
timestamps, and the launched PID where present to distinguish fresh evidence
from old records and output from other processes. Do not claim another instance's
appended lines prove this launch succeeded. If rotation/truncation makes the
boundary uncertain, disclose it instead of claiming complete coverage.

Apply [the analysis workflow](analyze-logs.md): cite actual source locations,
match descriptions, explain errors and diagnostics, retain unmatched messages,
and distinguish observed behavior from hypotheses. For the default smoke test,
verify that the owned process launched, survived the chosen interval, and
produced attributable runtime output. No fresh output or an early exit means
the live smoke test is failed or inconclusive, with the reason stated. This
does not establish that streaming, every configured service, or playback works.

On completion, failure, or cancellation, stop **only the still-running process
created and recorded by this test**, unless the user asked to leave it running.
Prefer normal termination and a bounded wait; do not use global `pkill`/`killall`
or signals based only on a PID read from an installation's PID file. If that
owned instance does not stop normally, report the situation and ask before a
forceful termination. Never stop an instance started by the user or another tool.

## 5. Report the test result

Report separately:

- Repository checks: command, actual pass/failure count, and relevant failures.
- Behavioral scenarios: cases exercised, observations, and those not run.
- Log integration: `observe` or `launch`, inspected files/time range, fresh versus
  historical coverage, source evidence, catalog interpretations, and uncertainty.
- For a launch: owned process identifier, duration, exit/cleanup status, and
  startup outcome. For observation: explicitly say no process interaction occurred.
- Pending checks, including missing directory/launch details or unavailable tools.

Save a requested test report under `.local/reports/`. A passed structural suite
does not convert a pending, failed, or inconclusive live check into a pass.
Commit-stage checks follow the same mode and ownership rules; a commit request
does not authorize interacting with a live installation or publishing changes.

## 6. Review repository contents for secrets before commit

When this workflow is part of commit preparation, do this after every required
repository and behavior test has finished and before committing. Review the
full repository working tree, including staged and unstaged changes, untracked
files, and hidden or ignored repository-local files that could be staged.
Check for private key or certificate files (such as `.pem`, `.key`, `.p12`, or
`.pfx`), passwords, tokens, API keys, credential-bearing connection strings,
personal notes, user-specific paths, actual Nimble logs, and `.local/` data.
Treat matches as prompts for review, since public certificates and synthetic
fixtures may be safe to keep. A local secret scanner can help find likely
credentials. Do not copy secret values into chat, reports, or command output;
record only paths and redacted categories. Keep sensitive and personal files
out of the commit and local to the user. Do not delete user files automatically.
