# Playlist audio packet was rejected

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-ce7a6d4c9e6f6444

## Log pattern

```text
ServerBlockProcessor: processAudioMessage failed for vod file [%s], stream=[%s/%s]
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.
- Argument 2 — Application: Text naming the application.
- Argument 3 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: processAudioMessage failed for vod file [/media/program.mp4], stream=[live/camera]
```

## Short description

Playlist audio packet was rejected.

## Trigger condition

Processing a source audio packet through the general AAC or MPEG audio path fails.

## Explanation

The block processor becomes failed and returns immediately; the playlist worker subsequently stops.

## Possible causes

- Processing a source audio packet through the general AAC or MPEG audio path fails.

## What to check

1. Check the source audio packet and preceding audio-processing errors.

## Recommended action

Check the source audio packet and preceding audio-processing errors.
