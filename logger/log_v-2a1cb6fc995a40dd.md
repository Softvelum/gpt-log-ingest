# RTMP playback selects an advertising item

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_v
Log level: verbose
Documentation ID: log_v-2a1cb6fc995a40dd

## Log pattern

```text
s=%d stream='%s/%s' play new content id='%s' uri='%s' stream_time=%llu
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.
- Argument 2 — Application: Playback application.
- Argument 3 — Stream: Playback stream name.
- Argument 4 — Content ID: Advertising content identifier.
- Argument 5 — Content URI: Configured URI of the selected advertising media item.
- Argument 6 — Stream time: Playback scheduling time in milliseconds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42 stream='live/camera' play new content id='ad-1' uri='https://media.example/ad.mp4' stream_time=60000
```

## Short description

RTMP playback selects an advertising item.

## Trigger condition

The first scheduled content item is due at or before the current playback stream time.

## Explanation

The item is removed from the pending schedule and selected for insertion. Loading and media-reader checks follow, so this message does not prove that media has started.

## Possible causes

- The first scheduled content item is due at or before the current playback stream time.

## What to check

1. Check the selected content identity, URI, and subsequent loading or start messages.

## Recommended action

No action is required when the scheduled item is expected.
