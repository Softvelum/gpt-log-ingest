# Child process wait result has no terminal-state flag

Product: Nimble Streamer
Component: Managed operating-system tasks
Logging function: log_d
Log level: debug
Documentation ID: log_d-d70183ced4fe300d

## Log pattern

```text
pid=%d is running
```

## Parameters

- Argument 1 — Process: Integer task process identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
pid=1234 is running
```

## Short description

Child process wait result has no terminal-state flag.

## Trigger condition

The process wait returns a positive identifier but none of the exited, stopped or signaled checks succeeds. Available only on Linux builds.

## Explanation

No task completion or restart is recorded by this branch.
