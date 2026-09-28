# AJA capture frame circulation initialization failed

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-d1bf904bc4d20e01

## Log pattern

```text
capture failed device=%s/%d reason='AutoCirculate input init failed'
```

## Parameters

- Argument 1 — Device: Configured AJA device selector or display name, as indicated by the message.
- Argument 2 — Channel: Numeric AJA channel identifier used by the device API; it is not converted to a one-based display number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
capture failed device=0/0 reason='AutoCirculate input init failed'
```

## Short description

AJA capture frame circulation initialization failed.

## Trigger condition

The card reports failure while initializing its input frame circulation.

## Explanation

The capture quit flag is set, no further capture loop runs, and input circulation is stopped during cleanup.

## Possible causes

- The card reports failure while initializing its input frame circulation.

## What to check

1. Check the capture channel, configured frame range, and device circulation state.

## Related log messages

- [AJA capture frame circulation start failed](log_e-2e2014bee79cbf5d.md)
