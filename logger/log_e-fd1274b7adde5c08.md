# NVIDIA host output frame allocation failed

Product: Nimble Streamer
Component: NVIDIA video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-fd1274b7adde5c08

## Log pattern

```text
failed to create video frame w:%d h:%d fmt:nv12 for [%s]
```

## Parameters

- Argument 1 — Width: Display width used for decoded output, integer pixels.
- Argument 2 — Height: Display height used for decoded output, integer pixels.
- Argument 3 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create video frame w:1920 h:1080 fmt:nv12 for [live/camera]
```

## Short description

NVIDIA host output frame allocation failed.

## Trigger condition

Allocating an NV12 output frame or its host pixel buffer returns no allocation.

## Explanation

Frame-queue processing returns failure before mapping the corresponding GPU frame.

## Possible causes

- Host frame or pixel-buffer allocation fails.

## What to check

1. Check the reported output dimensions and available host memory.

## Recommended action

Restore memory availability or reduce decoding load.
