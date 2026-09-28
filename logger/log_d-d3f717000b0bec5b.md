# Processed playlist text is available

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-d3f717000b0bec5b

## Log pattern

```text
parsed playlist:
%s
```

## Parameters

- Argument 1 — Processed playlist: Text after HLS URI or advertising changes, or DASH placeholder replacement.

## Log examples

Illustrative examples; these are not captured server logs.

```text
parsed playlist:
#EXTM3U
#EXT-X-TARGETDURATION:6
#EXT-X-MEDIA-SEQUENCE:1
#EXTINF:6.000,
segment1.ts

```

## Short description

Processed playlist text is available.

## Trigger condition

Playlist rewriting, VOD advertisement insertion, or cached playlist preparation has produced the displayed text.

## Explanation

The text is recorded before response construction or conditional-response handling; it does not prove delivery to the player.

## Possible causes

- Playlist rewriting, VOD advertisement insertion, or cached playlist preparation has produced the displayed text.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
