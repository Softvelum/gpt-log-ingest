# Managed task exceeded its output inactivity limit

Product: Nimble Streamer
Component: Managed operating-system tasks
Logging function: log_d
Log level: debug
Documentation ID: log_d-6b52ad4ff49fb0a1

## Log pattern

```text
restarting task id=%s pid=%d command='%s' because of no output
```

## Parameters

- Argument 1 — Task identifier: Text identifying the managed task.
- Argument 2 — Process: Integer task process identifier.
- Argument 3 — Command: Configured shell command text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
restarting task id=task1 pid=1234 command='echo ready' because of no output
```

## Short description

Managed task exceeded its output inactivity limit.

## Trigger condition

The configured no-output interval has elapsed for a task that is neither stopped nor failed. Available only on Linux builds.

## Explanation

The existing task is stopped and a new start is attempted immediately. Activity includes processed output events, so the message refers to the stored activity time.

## Possible causes

- The stored last-activity time is older than the configured no-output interval.

## What to check

1. Check the task output and its configured inactivity interval.
