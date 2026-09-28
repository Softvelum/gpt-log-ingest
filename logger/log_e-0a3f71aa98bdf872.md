# VPX input pixel format is unsupported

Product: Nimble Streamer
Component: VP8 and VP9 encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-0a3f71aa98bdf872

## Log pattern

```text
unsupported frame format=%d
```

## Parameters

- Argument 1 — Pixel format: FFmpeg pixel-format identifier, integer enumeration value; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported frame format=23
```

## Short description

VPX input pixel format is unsupported.

## Trigger condition

An input frame uses a pixel format other than planar YUV420P.

## Explanation

The frame is rejected before initialization or encoding.

## Possible causes

- The upstream pipeline supplied a different pixel layout.

## What to check

1. Check the FFmpeg pixel-format identifier and video conversion settings.

## Recommended action

Convert frames to planar YUV420P before this encoder.
