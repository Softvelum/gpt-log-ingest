# Playlist source packet could not be read

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-e59c29dad5b2d7eb

## Log pattern

```text
ServerBlockProcessor: readPacket failed for vod file url=[%s]
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: readPacket failed for vod file url=[/media/program.mp4]
```

## Short description

Playlist source packet could not be read.

## Trigger condition

The media reader reports failure while obtaining the next source packet.

## Explanation

The block processor becomes failed and returns immediately; the playlist worker subsequently stops.

## Possible causes

- The media reader reports failure while obtaining the next source packet.

## What to check

1. Check earlier media-open, seek, packet-read, or packet-conversion errors.

## Recommended action

Check earlier media-open, seek, packet-read, or packet-conversion errors.
