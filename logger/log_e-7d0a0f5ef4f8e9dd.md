# FFmpeg CUDA device creation failed

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-7d0a0f5ef4f8e9dd

## Log pattern

```text
failed to create shared hw device ctx for gpu=%s, err=%d
```

## Parameters

- Argument 1 — GPU: CUDA device index passed to FFmpeg as text.
- Argument 2 — FFmpeg error: FFmpeg negative error code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create shared hw device ctx for gpu=0, err=-22
```

## Short description

FFmpeg CUDA device creation failed.

## Trigger condition

No shared FFmpeg device context is cached for the requested GPU, and creating one returns a negative error.

## Explanation

The shared hardware-device request returns failure.

## Possible causes

- FFmpeg cannot create a CUDA hardware device for the supplied GPU selection.

## What to check

1. Check the FFmpeg error code, GPU index and CUDA driver availability.

## Recommended action

Correct GPU selection or driver support and retry.
