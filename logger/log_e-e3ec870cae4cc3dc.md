# Connected filter graph configuration failed

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-e3ec870cae4cc3dc

## Log pattern

```text
failed to configure filter graph, filter=%s, stream=%s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to configure filter graph, filter=[in]anull[out], stream=live/camera
```

## Short description

Connected filter graph configuration failed.

## Trigger condition

All inputs have been connected, but final FFmpeg graph configuration returns a negative result.

## Explanation

The audio or video graph is not marked ready and the frame write fails.

## Possible causes

- The connected filter graph cannot negotiate compatible parameters.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
