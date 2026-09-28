# Managed task stop begins

Product: Nimble Streamer
Component: Managed operating-system tasks
Logging function: log_d
Log level: debug
Documentation ID: log_d-7a3b34e122ab73bc

## Log pattern

```text
stopping task id='%s' pid=%d command='%s' outfd=%d
```

## Parameters

- Argument 1 — Task identifier: Text identifying the managed task.
- Argument 2 — Process: Integer task process identifier.
- Argument 3 — Command: Configured shell command text.
- Argument 4 — Output descriptor: Integer descriptor used by Nimble to read the task output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
stopping task id='task1' pid=1234 command='echo ready' outfd=12
```

## Short description

Managed task stop begins.

## Trigger condition

A nonempty task is passed to the stop routine. Available only on Linux builds.

## Explanation

Output monitoring and its descriptor are removed first. The process group receives a termination signal and is given five seconds to stop before forced termination is attempted.
