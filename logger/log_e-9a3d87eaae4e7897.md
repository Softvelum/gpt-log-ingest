# Compressed video forwarding fell behind

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_e
Log level: error
Documentation ID: log_e-9a3d87eaae4e7897

## Log pattern

```text
failed to read video in appropriate rate [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read video in appropriate rate [live/camera]
```

## Short description

Compressed video forwarding fell behind.

## Trigger condition

Reading the next live video item reports that its requested buffer position has expired.

## Explanation

The reader resets its position, skips interframes until a keyframe, and attempts to continue from an available item.

## Possible causes

- The output worker is slower than the retained input video stream.

## What to check

1. Check output processing capacity, host load, and buffer retention.

## Recommended action

Remove the processing bottleneck; this path already attempts keyframe recovery.
