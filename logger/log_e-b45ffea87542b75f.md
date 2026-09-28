# Input frame was rejected by the filter

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-b45ffea87542b75f

## Log pattern

```text
failed to add frame to filtergraph f='%s', stream=%s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to add frame to filtergraph f='[in]anull[out]', stream=live/camera
```

## Short description

Input frame was rejected by the filter.

## Trigger condition

An already configured audio or video graph returns an error when accepting a frame.

## Explanation

The frame-processing operation fails before reading filtered outputs.

## Possible causes

- FFmpeg rejected the input frame for this graph.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
