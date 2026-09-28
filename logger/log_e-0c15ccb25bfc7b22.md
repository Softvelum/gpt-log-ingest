# Video filter output frame allocation failed

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-0c15ccb25bfc7b22

## Log pattern

```text
failed to create video frame, out of memory, filter=%s, stream=%s
```

## Parameters

- Argument 1 — Filter: Complete configured video filter expression, text.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create video frame, out of memory, filter=[in]null[out], stream=live/camera
```

## Short description

Video filter output frame allocation failed.

## Trigger condition

Allocating a frame to receive video filter output returns no frame.

## Explanation

Video filter output processing fails.

## Possible causes

- The output frame allocation could not obtain storage.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
