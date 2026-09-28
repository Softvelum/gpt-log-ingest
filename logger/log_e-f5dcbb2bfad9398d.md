# Video filter hardware frame context was rejected

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-f5dcbb2bfad9398d

## Log pattern

```text
failed to set hw_frames_ctx filter=%s, stream=%s
```

## Parameters

- Argument 1 — Filter: Complete configured video filter expression, text.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set hw_frames_ctx filter=[in]null[out], stream=live/camera
```

## Short description

Video filter hardware frame context was rejected.

## Trigger condition

A hardware input supplies a frame context, but assigning it to the source filter fails.

## Explanation

Video input setup fails.

## Possible causes

- FFmpeg rejected the supplied hardware frame context.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
