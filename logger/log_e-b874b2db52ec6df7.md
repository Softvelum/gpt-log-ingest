# HEVC decoder creation failure guard

Product: Nimble Streamer
Component: Video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-b874b2db52ec6df7

## Log pattern

```text
failed to create hevc decoder for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create hevc decoder for [live/camera]
```

## Short description

HEVC decoder creation failure guard.

## Trigger condition

The HEVC decoder-selection helper returns failure during live source selection or resolution-change replacement.

## Explanation

The caller would reject the source update or packet and stop its worker. In the current implementation this helper always returns success after choosing a decoder, so this guarded log has no ordinary reachable failure path.

## Possible causes

- No runtime failure is returned by the current selection helper; hardware selection failures instead fall back to FFmpeg.

## What to check

1. If this message is observed, verify the running build and retain preceding decoder diagnostics.

## Recommended action

Investigate the running build and decoder-selection result before changing the input.
