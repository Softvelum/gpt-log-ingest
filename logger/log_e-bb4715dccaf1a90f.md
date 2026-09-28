# File descriptor close failed

Product: Nimble Streamer
Component: Operating system utilities
Logging function: log_e
Log level: error
Documentation ID: log_e-bb4715dccaf1a90f

## Log pattern

```text
failed to close file fd=%d (errno=%d)
```

## Parameters

- Argument 1 — Descriptor: Open file descriptor, integer.
- Argument 2 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to close file fd=12 (errno=5)
```

## Short description

File descriptor close failed.

## Trigger condition

The platform file-close operation returns an error.

## Explanation

The helper returns failure.

## Possible causes

- The operating system rejected file closure.

## What to check

1. Check the descriptor and error code.

## Recommended action

Investigate the close failure.
