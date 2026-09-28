# Playlist VP8 initialization failed

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-19e5d14aec14b2d3

## Log pattern

```text
ServerBlockProcessor: failed to set vp8 codec for vod file url=[%s]
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: failed to set vp8 codec for vod file url=[/media/program.mp4]
```

## Short description

Playlist VP8 initialization failed.

## Trigger condition

The source has no video configuration data and initializing VP8 output returns failure.

## Explanation

The block processor becomes failed and returns immediately; the playlist worker subsequently stops.

## Possible causes

- The source has no video configuration data and initializing VP8 output returns failure.

## What to check

1. Check the source VP8 stream and prior output initialization errors.

## Recommended action

Check the source VP8 stream and prior output initialization errors.
