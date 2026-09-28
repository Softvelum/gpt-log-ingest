# Cannot set SRT maximum bandwidth

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-b53f7b4aa36fc37b

## Log pattern

```text
failed to set maxbw s=%d
```

## Parameters

- Argument 1 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set maxbw s=100
```

## Short description

Cannot set SRT maximum bandwidth.

## Trigger condition

The SRT library rejects the configured maximum bandwidth value.

## Explanation

Applying the socket settings stops with failure; callers creating a socket or group then close it.

## Possible causes

- The SRT library rejects the configured maximum bandwidth value.

## What to check

1. Check the configured maximum bandwidth and nearby SRT diagnostics.
