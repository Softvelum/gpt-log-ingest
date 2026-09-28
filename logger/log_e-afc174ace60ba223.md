# Playlist local media file does not exist

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-afc174ace60ba223

## Log pattern

```text
ServerBlockProcessor: vod file url=[%s] doesn't exist
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: vod file url=[/media/program.mp4] doesn't exist
```

## Short description

Playlist local media file does not exist.

## Trigger condition

The local media path fails the existence check.

## Explanation

No reader is created. The caller advances to the next scheduled stream when this source cannot be opened.

## Possible causes

- The local media path fails the existence check.

## What to check

1. Correct the media path or restore the missing file.

## Recommended action

Correct the media path or restore the missing file.
