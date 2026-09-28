# MPEG-2 final picture segment is empty

Product: Nimble Streamer
Component: Video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-9db6e8e03e27dc80

## Log pattern

```text
invalid mpeg2video packet size for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid mpeg2video packet size for [live/camera]
```

## Short description

MPEG-2 final picture segment is empty.

## Trigger condition

After splitting a live MPEG-2 message, the remaining length from the last packet start to the message end is nonpositive.

## Explanation

The final segment is rejected before decoding and the video worker stops.

## Possible causes

- The final picture boundary is inconsistent with the message extent; this is a defensive size check.

## What to check

1. Inspect the failing message extent and MPEG-2 picture start codes.

## Recommended action

Correct malformed input or investigate the reproducible boundary inconsistency.
