# NVIDIA picture decoding failed

Product: Nimble Streamer
Component: NVIDIA video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-c2badcaf877c951d

## Log pattern

```text
failed to decode picture for [%s], res=%d
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — CUDA status: CUDA or NVIDIA video-decoder driver result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to decode picture for [live/camera], res=2
```

## Short description

NVIDIA picture decoding failed.

## Trigger condition

Submitting parsed picture parameters to the hardware decoder returns an error.

## Explanation

The decode callback marks an error; the current packet operation fails.

## Possible causes

- The decoder rejects the picture or cannot execute its decoding operation.

## What to check

1. Check the driver status, encoded input integrity and GPU state.

## Recommended action

Correct invalid input or restore the decoder/device.
