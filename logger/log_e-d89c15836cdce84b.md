# Buffered NDI output pixel format is unsupported

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-d89c15836cdce84b

## Log pattern

```text
unsupported video format=%d, add format=bgra|uyvy422|nv12|yuv420p into video pipeline
```

## Parameters

- Argument 1 — Pixel format: Integer media-processing pixel-format identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported video format=-1, add format=bgra|uyvy422|nv12|yuv420p into video pipeline
```

## Short description

Buffered NDI output pixel format is unsupported.

## Trigger condition

A frame uses a pixel format other than BGRA, UYVY422, NV12, or YUV420P.

## Explanation

The video sender breaks its worker loop before sending that frame and then logs that the video sender stopped.

## Possible causes

- A frame uses a pixel format other than BGRA, UYVY422, NV12, or YUV420P.

## What to check

1. Check the pixel format supplied by the video pipeline.

## Recommended action

Add a video format conversion to bgra, uyvy422, nv12, or yuv420p.

## Related log messages

- [Buffered NDI video sender stopped](log_i-493e497c7a6f3212.md)
