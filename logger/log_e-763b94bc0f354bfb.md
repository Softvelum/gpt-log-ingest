# Playlist video configuration data is absent

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-763b94bc0f354bfb

## Log pattern

```text
ServerBlockProcessor: failed to read video extradata for vod file url=[%s]
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: failed to read video extradata for vod file url=[/media/program.mp4]
```

## Short description

Playlist video configuration data is absent.

## Trigger condition

Video configuration data is empty for a source codec other than VP8 or VP9.

## Explanation

The block processor becomes failed and returns immediately; the playlist worker subsequently stops.

## Possible causes

- Video configuration data is empty for a source codec other than VP8 or VP9.

## What to check

1. Check that the source file contains valid codec configuration data.

## Recommended action

Check that the source file contains valid codec configuration data.
