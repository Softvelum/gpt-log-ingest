# Playlist AV1, VP8, or VP9 packet was rejected

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-8c06310fcc52ddab

## Log pattern

```text
ServerBlockProcessor: processVideoMessageAV1 failed for vod file [%s], stream=[%s/%s]
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.
- Argument 2 — Application: Text naming the application.
- Argument 3 — Stream: Text naming the stream.

The original message names AV1 in all three codec branches.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: processVideoMessageAV1 failed for vod file [/media/program.mp4], stream=[live/camera]
```

## Short description

Playlist AV1, VP8, or VP9 packet was rejected.

## Trigger condition

Forwarding an AV1, VP8, or VP9 source packet to the output fails.

## Explanation

The block processor becomes failed and returns immediately; the playlist worker subsequently stops.

## Possible causes

- Forwarding an AV1, VP8, or VP9 source packet to the output fails.

## What to check

1. Check the actual source codec and preceding video-processing errors.

## Recommended action

Check the actual source codec and preceding video-processing errors.
