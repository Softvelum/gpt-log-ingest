# Cannot set SRT flow-control window

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-f6c1dee9426bc188

## Log pattern

```text
failed to set fc=%s, s=%d
```

## Parameters

- Argument 1 — Value: Original configured flow-control window string, passed numerically without unit conversion.
- Argument 2 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set fc=256, s=100
```

## Short description

Cannot set SRT flow-control window.

## Trigger condition

The SRT library rejects the configured flow-control window.

## Explanation

Applying socket settings stops with failure. The logged value is the original text converted to an integer before it is passed to SRT.

## Possible causes

- The SRT library rejects the configured flow-control window.

## What to check

1. Check the configured flow-control window and library compatibility.
