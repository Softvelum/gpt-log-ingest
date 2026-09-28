# Managed task pipe creation failed

Product: Nimble Streamer
Component: Managed operating-system tasks
Logging function: log_e
Log level: error
Documentation ID: log_e-1353afce2e6406ca

## Log pattern

```text
pipe() failed for task='%s' errno=%d
```

## Parameters

- Argument 1 — Command: Configured shell command text.
- Argument 2 — System error: Integer operating-system error value; it may be stale unless the preceding operation failed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
pipe() failed for task='echo ready' errno=24
```

## Short description

Managed task pipe creation failed.

## Trigger condition

Creating either the task input pipe or output pipe fails. Available only on Linux builds.

## Explanation

Subprocess creation returns failure. If the second pipe fails, both ends of the first pipe are closed.

## Possible causes

- The operating system rejected pipe creation.

## What to check

1. Inspect the system error and process descriptor availability.
