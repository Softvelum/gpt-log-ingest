# SDI output context allocation failed

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-c36bffe01a4840c3

## Log pattern

```text
cannot alloc output context [%s/%s]
```

## Parameters

- Argument 1 — Device type: FFmpeg output device type, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot alloc output context [decklink/live/camera]
```

## Short description

SDI output context allocation failed.

## Trigger condition

Allocating an output format context returns a negative result.

## Explanation

The device output creation request returns failure.

## Possible causes

- FFmpeg cannot allocate the requested device output context.

## What to check

1. Check memory availability and device-format support.

## Recommended action

Restore resources or correct device support and retry.
