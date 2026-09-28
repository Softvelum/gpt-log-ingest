# HLS playlist received for processing

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_d
Log level: debug
Documentation ID: log_d-5caf869a4c1f849b

## Log pattern

```text
received playlist url='%s' len=%u
```

## Parameters

- Argument 1 — url: Current origin URL.
- Argument 2 — playlist\_bytes: Playlist string length in bytes, explicitly converted to an unsigned 32-bit integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
received playlist url='https://origin.example/live.m3u8' len=240
```

## Short description

HLS playlist received for processing.

## Trigger condition

The complete response body reaches playlist processing.

## Explanation

Queued chunks for the current media identifier may first be cleared. The playlist byte count is then logged before media tags, encryption, sequence numbers, and child playlists are interpreted.
