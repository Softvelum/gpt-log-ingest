# Filter output time base is unavailable

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-3cffdd5c7e329539

## Log pattern

```text
failed to get time_base, filter=%s, stream=%s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get time_base, filter=[in]anull[out], stream=live/camera
```

## Short description

Filter output time base is unavailable.

## Trigger condition

The output sink has no input-link array or no first input link when a filtered frame is ready.

## Explanation

The audio or video frame is not forwarded because its output time base cannot be obtained.

## Possible causes

- The output sink lacks the expected connected input.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
