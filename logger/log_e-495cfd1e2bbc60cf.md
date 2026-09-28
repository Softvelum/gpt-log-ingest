# Quick Sync encoder received an unsupported pixel format

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-495cfd1e2bbc60cf

## Log pattern

```text
%p invalid frame format %d
```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.
- Argument 2 — Pixel format: FFmpeg pixel-format identifier, integer enumeration value; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 invalid frame format 0
```

## Short description

Quick Sync encoder received an unsupported pixel format.

## Trigger condition

An input frame is not in NV12 pixel format when a media surface is requested.

## Explanation

The frame cannot be converted into an input surface and writing it fails.

## Possible causes

- The upstream video pipeline supplied a different pixel format.

## What to check

1. Check the reported FFmpeg pixel format and upstream conversion settings.

## Recommended action

Convert frames to NV12 before passing them to this encoder.
