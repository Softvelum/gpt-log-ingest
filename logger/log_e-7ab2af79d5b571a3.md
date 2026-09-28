# H.264 decoder creation failure guard

Product: Nimble Streamer
Component: Video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-7ab2af79d5b571a3

## Log pattern

```text
failed to create avc decoder for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create avc decoder for [live/camera]
```

## Short description

H.264 decoder creation failure guard.

## Trigger condition

The H.264 decoder-selection helper returns failure during file input selection.

## Explanation

The caller would reject the source update or packet and stop its worker. In the current implementation this helper always returns success after choosing a decoder, so this guarded log has no ordinary reachable failure path.

## Possible causes

- No runtime failure is returned by the current selection helper; hardware selection failures instead fall back to FFmpeg.

## What to check

1. If this message is observed, verify the running build and retain preceding decoder diagnostics.

## Recommended action

Investigate the running build and decoder-selection result before changing the input.
