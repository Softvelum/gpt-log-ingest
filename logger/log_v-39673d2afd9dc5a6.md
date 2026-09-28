# Managed task state reported after termination request

Product: Nimble Streamer
Component: Managed operating-system tasks
Logging function: log_v
Log level: verbose
Documentation ID: log_v-39673d2afd9dc5a6

## Log pattern

```text
task pid=%d cmd='%s' is exited status=%d errno=%d wifexited=%d wifstopped=%d wifsignaled=%d with SIGTERM
```

## Parameters

- Argument 1 — Process: Integer task process identifier.
- Argument 2 — Command: Configured shell command text.
- Argument 3 — Exit status: Exit-status field extracted from the process wait result; meaningful as an exit code only when normal exit is indicated.
- Argument 4 — System error: Integer operating-system error value; it may be stale unless the preceding operation failed.
- Argument 5 — Exited: Nonzero when normal process exit was reported.
- Argument 6 — Stopped: Nonzero when a stopped process was reported.
- Argument 7 — Signaled: Nonzero when termination by signal was reported.

## Log examples

Illustrative examples; these are not captured server logs.

```text
task pid=1234 cmd='echo ready' is exited status=0 errno=0 wifexited=1 wifstopped=0 wifsignaled=0 with SIGTERM
```

## Short description

Managed task state reported after termination request.

## Trigger condition

A process wait reports exit, signal termination or a stopped state within the five-second wait following a termination request. Available only on Linux builds.

## Explanation

The task is marked as being stopped. The state flags distinguish normal exit, stopped state and signal termination; the printed exit-status field alone does not establish the outcome.
