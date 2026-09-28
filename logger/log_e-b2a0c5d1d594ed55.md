# Playlist MPEG-TS video codec is unsupported

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-b2a0c5d1d594ed55

## Log pattern

```text
unsupported mpegts video codec=%d, file=%s
```

## Parameters

- Argument 1 — Codec number: Integer media-library video codec identifier.
- Argument 2 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported mpegts video codec=226, file=/media/program.mp4
```

## Short description

Playlist MPEG-TS video codec is unsupported.

## Trigger condition

A selected MPEG-TS video packet uses a codec other than H.264 or H.265.

## Explanation

Packet reading returns failure because this conversion path supports only those two codecs.

## Possible causes

- A selected MPEG-TS video packet uses a codec other than H.264 or H.265.

## What to check

1. Use H.264 or H.265 video for this MPEG-TS playlist input.

## Recommended action

Use H.264 or H.265 video for this MPEG-TS playlist input.
