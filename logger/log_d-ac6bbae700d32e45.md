# RTMP audio-only advertising timing is established

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-ac6bbae700d32e45

## Log pattern

```text
s=%d, %s content start, audio only, start_time=%lld, initial_audio_timestamp=%lld
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.
- Argument 2 — Content URI: Configured URI of the selected advertising media item.
- Argument 3 — Start time: Advertisement start in the playback stream timeline, in milliseconds.
- Argument 4 — Initial audio timestamp: Content starting timestamp rescaled to milliseconds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, https://media.example/ad.mp4 content start, audio only, start_time=60000, initial_audio_timestamp=120000
```

## Short description

RTMP audio-only advertising timing is established.

## Trigger condition

A loaded audio-only advertising reader is assigned the stream insertion timestamp.

## Explanation

The content start is recorded on the playback stream timeline; metadata and any sequence headers are sent next.

## Possible causes

- A loaded audio-only advertising reader is assigned the stream insertion timestamp.

## What to check

1. Check the audio timestamp against the intended insertion point.

## Recommended action

No action is required for expected advertising playback.
