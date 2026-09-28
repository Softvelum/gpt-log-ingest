# Filtered audio could not be read

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-fa5eac91390adf25

## Log pattern

```text
failed to get filtered frame, filter=%s, stream=%s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get filtered frame, filter=[in]anull[out], stream=live/camera
```

## Short description

Filtered audio could not be read.

## Trigger condition

Reading an audio output frame returns an error other than need-more-input or end-of-stream.

## Explanation

The frame-processing operation fails.

## Possible causes

- FFmpeg reported a filtering output failure.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
