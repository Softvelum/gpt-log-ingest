# SDI output video stream allocation failed

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-bbc08ebc6b340d34

## Log pattern

```text
cannot create video stream [%s/%s]
```

## Parameters

- Argument 1 — Device type: FFmpeg output device type, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot create video stream [decklink/live/camera]
```

## Short description

SDI output video stream allocation failed.

## Trigger condition

Creating the required output video stream returns no stream.

## Explanation

The device output creation request returns failure.

## Possible causes

- FFmpeg cannot allocate a video stream in the output context.

## What to check

1. Check memory availability while creating the device output.

## Recommended action

Restore available memory and retry.
