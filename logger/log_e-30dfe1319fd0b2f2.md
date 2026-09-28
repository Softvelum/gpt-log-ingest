# Playlist Opus audio packet was rejected

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-30dfe1319fd0b2f2

## Log pattern

```text
ServerBlockProcessor: processOpusAudioMessage failed for vod file [%s], stream=[%s/%s]
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.
- Argument 2 — Application: Text naming the application.
- Argument 3 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: processOpusAudioMessage failed for vod file [/media/program.mp4], stream=[live/camera]
```

## Short description

Playlist Opus audio packet was rejected.

## Trigger condition

Passing an Opus packet from the file into the output stream fails.

## Explanation

The block processor becomes failed and returns immediately; the playlist worker subsequently stops.

## Possible causes

- Passing an Opus packet from the file into the output stream fails.

## What to check

1. Check source Opus packet data and preceding audio-processing errors.

## Recommended action

Check source Opus packet data and preceding audio-processing errors.
