# SDI output audio stream allocation failed

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-0b9ccacdbfe61b69

## Log pattern

```text
cannot create audio stream [%s/%s]
```

## Parameters

- Argument 1 — Device type: FFmpeg output device type, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot create audio stream [decklink/live/camera]
```

## Short description

SDI output audio stream allocation failed.

## Trigger condition

Audio is configured but creating the output audio stream returns no stream.

## Explanation

The device output creation request returns failure.

## Possible causes

- FFmpeg cannot allocate an audio stream in the output context.

## What to check

1. Check memory availability while creating the device output.

## Recommended action

Restore available memory and retry.
