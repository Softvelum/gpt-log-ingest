# Child forced-kill wait result

Product: Nimble Streamer
Component: Operating system utilities
Logging function: log_d
Log level: debug
Documentation ID: log_d-11c4479fdac01eab

## Log pattern

```text
SIGKILL pid=%d errno=%d wifexited=%d wifstopped=%d wifsignaled=%d
```

## Parameters

- Argument 1 — Wait result: Process identifier returned by the child-status wait; -1 indicates a wait error.
- Argument 2 — Error state: Current errno integer; it can be stale when the wait succeeds.
- Argument 3 — Exited: Nonzero if wait status indicates normal exit, integer.
- Argument 4 — Stopped: Nonzero if wait status indicates a stopped child, integer.
- Argument 5 — Signaled: Nonzero if wait status indicates termination by signal, integer.

The process field is the wait return value. The error code can be stale on success; decoded status alone is unreliable after a failed wait.

## Log examples

Illustrative examples; these are not captured server logs.

```text
SIGKILL pid=1234 errno=0 wifexited=1 wifstopped=0 wifsignaled=0
```

## Short description

Child forced-kill wait result.

## Trigger condition

After sending a forced-kill signal, the non-Windows blocking wait returns.

## Explanation

The wait result and decoded status are logged before the final process-library wait. The log is emitted even if the wait result is an error.

## Related log messages

- [Child forced-kill request started](log_d-ec783318a83f7e8f.md)
