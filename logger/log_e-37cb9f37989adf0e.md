# Cannot set SRT send buffer

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-37cb9f37989adf0e

## Log pattern

```text
failed to set sndbuf=%s, s=%d
```

## Parameters

- Argument 1 — Value: Original configured send-buffer size string, passed numerically without unit conversion.
- Argument 2 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set sndbuf=4096, s=100
```

## Short description

Cannot set SRT send buffer.

## Trigger condition

The SRT library rejects the configured send-buffer size.

## Explanation

Applying socket settings stops with failure. The logged value is the original text converted to an integer before it is passed to SRT.

## Possible causes

- The SRT library rejects the configured send-buffer size.

## What to check

1. Check the configured send-buffer size and library compatibility.
