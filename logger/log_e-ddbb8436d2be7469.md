# NVIDIA decoded frame mapping failed

Product: Nimble Streamer
Component: NVIDIA video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ddbb8436d2be7469

## Log pattern

```text
failed to map video frame for [%s], res=%d
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — CUDA status: CUDA or NVIDIA video-decoder driver result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to map video frame for [live/camera], res=2
```

## Short description

NVIDIA decoded frame mapping failed.

## Trigger condition

Mapping a queued decoded picture into GPU addressable output memory returns an error.

## Explanation

Frame-queue processing fails and the picture is not delivered to downstream writers.

## Possible causes

- The decoder cannot map the selected picture for output.

## What to check

1. Check the driver result and decoder/device state.

## Recommended action

Correct the mapping failure or restart the decoder.
