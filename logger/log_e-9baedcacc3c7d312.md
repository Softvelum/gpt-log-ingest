# SDI video converter allocation failed

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-9baedcacc3c7d312

## Log pattern

```text
failed to create video converter context for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create video converter context for [live/camera]
```

## Short description

SDI video converter allocation failed.

## Trigger condition

Allocating the output video encoder context returns no context.

## Explanation

The current output frame operation returns failure.

## Possible causes

- FFmpeg cannot allocate the encoder context.

## What to check

1. Check memory availability during device output startup.

## Recommended action

Restore available memory and retry.
