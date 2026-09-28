# SDI output initialization result is unexpected

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-db1c95ab84ad166c

## Log pattern

```text
failed to initialize [%s/%s]
```

## Parameters

- Argument 1 — Device type: FFmpeg output device type, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to initialize [decklink/live/camera]
```

## Short description

SDI output initialization result is unexpected.

## Trigger condition

Device output initialization returns a result other than the accepted initialized or header-required states.

## Explanation

The device output creation request returns failure.

## Possible causes

- FFmpeg returns an unexpected output initialization state.

## What to check

1. Check the device support in the installed FFmpeg libraries.

## Recommended action

Use compatible FFmpeg device libraries and retry.
