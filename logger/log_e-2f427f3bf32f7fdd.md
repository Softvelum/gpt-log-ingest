# Playlist AAC channel count is unsupported

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-2f427f3bf32f7fdd

## Log pattern

```text
unsupported aac channels=%d, file=%s
```

## Parameters

- Argument 1 — Channels: Integer audio channel count.
- Argument 2 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported aac channels=7, file=/media/program.mp4
```

## Short description

Playlist AAC channel count is unsupported.

## Trigger condition

MPEG-TS AAC has a channel count outside one through six or eight.

## Explanation

Building AAC configuration data returns failure.

## Possible causes

- MPEG-TS AAC has a channel count outside one through six or eight.

## What to check

1. Use an AAC channel configuration with one through six channels or eight channels.

## Recommended action

Use an AAC channel configuration with one through six channels or eight channels.
