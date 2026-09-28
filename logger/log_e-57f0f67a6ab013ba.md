# Requested FFmpeg video encoder is unavailable

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-57f0f67a6ab013ba

## Log pattern

```text
failed to find %s video encoder for [%s]
```

## Parameters

- Argument 1 — Encoder name: Requested FFmpeg encoder name, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to find h264_nvenc video encoder for [live/camera]
```

## Short description

Requested FFmpeg video encoder is unavailable.

## Trigger condition

Looking up the requested FFmpeg encoder by name returns no codec.

## Explanation

Video initialization stops before creating an encoder context.

## Possible causes

- The loaded FFmpeg library does not contain the requested named encoder.

## What to check

1. Check the encoder name and compiled codec support in the installed transcoding libraries.

## Recommended action

Select or install the required supported encoder.
