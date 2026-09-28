# Playlist H.264 or H.265 video packet was rejected

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-4cc98b0d65cb80d4

## Log pattern

```text
ServerBlockProcessor: processVideoMessage failed for vod file [%s], stream=[%s/%s]
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.
- Argument 2 — Application: Text naming the application.
- Argument 3 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: processVideoMessage failed for vod file [/media/program.mp4], stream=[live/camera]
```

## Short description

Playlist H.264 or H.265 video packet was rejected.

## Trigger condition

Forwarding an H.264 or H.265 packet with its decode and presentation timing fails.

## Explanation

The block processor becomes failed and returns immediately; the playlist worker subsequently stops.

## Possible causes

- Forwarding an H.264 or H.265 packet with its decode and presentation timing fails.

## What to check

1. Check video packet structure, configuration, and preceding output errors.

## Recommended action

Check video packet structure, configuration, and preceding output errors.
