# NVIDIA input pixel conversion failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-56b3b0dcfb25d53b

## Log pattern

```text
failed to conv frame format %d->%d for [%s]
```

## Parameters

- Argument 1 — Source format: Initial FFmpeg input pixel-format identifier, integer.
- Argument 2 — Destination format: Chosen FFmpeg conversion pixel-format identifier, integer.
- Argument 3 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to conv frame format 0->23 for [live/camera]
```

## Short description

NVIDIA input pixel conversion failed.

## Trigger condition

A configured software pixel-format conversion fails after the NVIDIA input buffer has been locked.

## Explanation

The current frame is not submitted and the frame write returns failure.

## Possible causes

- Allocating the converted frame or pixels, copying frame properties, or converting image data fails.

## What to check

1. Check the source and destination pixel formats, frame validity and host memory.

## Recommended action

Restore conversion resources or use an input format accepted directly by the encoder.
