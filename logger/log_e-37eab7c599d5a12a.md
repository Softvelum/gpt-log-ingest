# Could not enter background service mode

Product: Nimble Streamer
Component: Server startup
Logging function: log_e
Log level: error
Documentation ID: log_e-37eab7c599d5a12a

## Log pattern

```text
cannot daemonize %d
```

## Parameters

- Argument 1 — System error: Decimal operating-system error number from the failed daemon operation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot daemonize 11
```

## Short description

Could not enter background service mode.

## Trigger condition

Background mode is enabled on a non-Windows build and the operating-system daemon operation fails.

## Explanation

The process exits with a failure status; server execution does not begin.

## Possible causes

- The operating system rejected the background-mode operation.

## What to check

1. Inspect the reported operating-system error number.

## Related log messages

- [Server entered background mode](log_i-e11ef73592e6a2dc.md)
