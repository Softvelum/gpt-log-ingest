# Video encoder input pixel format changed

Product: Nimble Streamer
Component: Video encoder input stability
Logging function: log_e
Log level: error
Documentation ID: log_e-8965da8fa3fa4b15

## Log pattern

```text
failed to change frame format from %d to %d on the fly for [%s]
```

## Parameters

- Argument 1 — Old format: Initial FFmpeg source pixel-format identifier, integer.
- Argument 2 — New format: Incoming FFmpeg pixel-format identifier, integer.
- Argument 3 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to change frame format from 0 to 23 on the fly for [live/camera]
```

## Short description

Video encoder input pixel format changed.

## Trigger condition

An initialized FFmpeg or native NVIDIA encoder receives a pixel format different from its initial input format.

## Explanation

The frame is rejected before pixel conversion or submission; existing conversion does not authorize changing its source format.

## Possible causes

- The decoder or filter changes the source pixel format during encoding.

## What to check

1. Compare the two FFmpeg format identifiers and upstream changes.

## Recommended action

Keep the source pixel format stable or restart encoding for the new format.
