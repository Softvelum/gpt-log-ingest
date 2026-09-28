# NV12 frame conversion failed

Product: Nimble Streamer
Component: x264 H.264 encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-820c0dfeae053e02

## Log pattern

```text
failed to convert frame: %d->nv12 for [%s]
```

## Parameters

- Argument 1 — Format: FFmpeg sample or pixel format identifier, integer; an enumeration rather than a bit depth.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to convert frame: 0->nv12 for [live/camera]
```

## Short description

NV12 frame conversion failed.

## Trigger condition

The x264 input format requires conversion and running the NV12 converter fails.

## Explanation

The frame is not passed to x264 and the write reports failure.

## Possible causes

- The input frame cannot be converted to NV12 by the current conversion path.

## What to check

1. Check the input format and conversion diagnostics.

## Recommended action

Supply a directly supported pixel format or correct the conversion failure.
