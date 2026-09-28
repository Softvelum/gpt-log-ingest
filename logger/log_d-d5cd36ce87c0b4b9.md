# Completed child is not a managed task

Product: Nimble Streamer
Component: Managed operating-system tasks
Logging function: log_d
Log level: debug
Documentation ID: log_d-d5cd36ce87c0b4b9

## Log pattern

```text
pid=%d is unknown
```

## Parameters

- Argument 1 — Process: Integer task process identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
pid=1234 is unknown
```

## Short description

Completed child is not a managed task.

## Trigger condition

A terminal process-wait result has no matching process identifier in the managed task table. Available only on Linux builds.

## Explanation

No managed task record is changed for this process.
