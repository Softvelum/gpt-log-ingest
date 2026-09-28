# Child shutdown status observed

Product: Nimble Streamer
Component: Operating system utilities
Logging function: log_d
Log level: debug
Documentation ID: log_d-ba77ba7433ae4a04

## Log pattern

```text
child pid=%d errno=%d wifexited=%d wifstopped=%d wifsignaled=%d stopped
```

## Parameters

- Argument 1 — Wait result: Process identifier returned by the child-status wait; -1 indicates a wait error.
- Argument 2 — Error state: Current errno integer; it can be stale when the wait succeeds.
- Argument 3 — Exited: Nonzero if wait status indicates normal exit, integer.
- Argument 4 — Stopped: Nonzero if wait status indicates a stopped child, integer.
- Argument 5 — Signaled: Nonzero if wait status indicates termination by signal, integer.

The error value is not proof of failure on this successful wait.

## Log examples

Illustrative examples; these are not captured server logs.

```text
child pid=1234 errno=0 wifexited=1 wifstopped=0 wifsignaled=0 stopped
```

## Short description

Child shutdown status observed.

## Trigger condition

On non-Windows systems, a nonblocking wait returns a child identifier and reports exited, signaled, or stopped status.

## Explanation

The timed polling loop ends without issuing the forced-kill request. A final process-library wait follows.

## Related log messages

- [Child forced-kill wait result](log_d-11c4479fdac01eab.md)
- [Child forced-kill request started](log_d-ec783318a83f7e8f.md)
