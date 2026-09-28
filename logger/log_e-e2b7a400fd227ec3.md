# Cannot set SRT maximum retransmission bandwidth

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-e2b7a400fd227ec3

## Log pattern

```text
failed to set maxrexmitbw=%s, s=%d
```

## Parameters

- Argument 1 — Value: Original configured maximum retransmission bandwidth string, passed numerically without unit conversion.
- Argument 2 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set maxrexmitbw=1000000, s=100
```

## Short description

Cannot set SRT maximum retransmission bandwidth.

## Trigger condition

The SRT library rejects the configured maximum retransmission bandwidth.

## Explanation

Applying socket settings stops with failure. The logged value is the original text converted to an integer before it is passed to SRT.

## Possible causes

- The SRT library rejects the configured maximum retransmission bandwidth.

## What to check

1. Check the configured maximum retransmission bandwidth and library compatibility.
