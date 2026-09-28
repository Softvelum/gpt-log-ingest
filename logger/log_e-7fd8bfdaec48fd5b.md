# DVR thumbnail codec is unsupported

Product: Nimble Streamer
Component: DVR
Logging function: log_e
Log level: error
Documentation ID: log_e-7fd8bfdaec48fd5b

## Log pattern

```text
failed to create video preview segment, unsupported video_codec=%d
```

## Parameters

- Argument 1 — Codec: Integer MP4 video codec identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create video preview segment, unsupported video_codec=0
```

## Short description

DVR thumbnail codec is unsupported.

## Trigger condition

The recorded video sample description is not AVC, HEVC, or AV1.

## Explanation

MP4 thumbnail creation returns failure.

## Possible causes

- The recorded video sample description is not AVC, HEVC, or AV1.

## What to check

1. Use a supported video codec for this thumbnail path.

## Recommended action

Use a supported video codec for this thumbnail path.
