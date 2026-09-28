# NVIDIA decoded luma copy failed

Product: Nimble Streamer
Component: NVIDIA video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-fcb3cd4b28e93a81

## Log pattern

```text
failed to copy Y plane for [%s], res=%d
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — CUDA status: CUDA or NVIDIA video-decoder driver result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to copy Y plane for [live/camera], res=2
```

## Short description

NVIDIA decoded luma copy failed.

## Trigger condition

Copying the NV12 luma plane from GPU memory into the host frame returns a driver error.

## Explanation

The frame copy returns failure after attempting to unlock any shared context; this frame is not delivered downstream.

## Possible causes

- The GPU-to-host copy cannot complete for the mapped frame.

## What to check

1. Check the CUDA status, preceding frame-map errors and device/memory availability.

## Recommended action

Restore valid device and host buffers or restart the failed decoder.

## Related log messages

- [NVIDIA decoded frame transfer failed](log_e-79157f773bb28fbe.md)
