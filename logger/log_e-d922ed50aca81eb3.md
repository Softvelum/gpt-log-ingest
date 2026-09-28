# Constant-bitrate SRT send failed

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-d922ed50aca81eb3

## Log pattern

```text
failed to send s=%d errno=%d: %s
```

## Parameters

- Argument 1 — Socket: SRT socket identifier.
- Argument 2 — Error number: SRT library error code.
- Argument 3 — Error: SRT library error description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send s=101 errno=2001: Connection was broken
```

## Short description

Constant-bitrate SRT send failed.

## Trigger condition

An SRT message send fails with an error other than asynchronous-send-wait or timeout.

## Explanation

The constant-bitrate send operation reports failure. Asynchronous-send-wait and timeout take a separate path and are not reported here. This sender is available on non-Windows builds.

## Possible causes

- An SRT message send fails with an error other than asynchronous-send-wait or timeout.

## What to check

1. Inspect the SRT error and receiver connection health.
