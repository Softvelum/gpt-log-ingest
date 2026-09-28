# Playlist media cannot be opened

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-9be2babe5bbc2959

## Log pattern

```text
failed to open file '%s' error='%s'
```

## Parameters

- Argument 1 — Content location: Text identifying a local media file or a remote cache key.
- Argument 2 — Open error: Text describing the media-open failure.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open file '/media/program.mp4' error='Invalid data found when processing input'
```

## Short description

Playlist media cannot be opened.

## Trigger condition

Opening the media input returns an error.

## Explanation

The reader reports failure with the media library error text.

## Possible causes

- Opening the media input returns an error.

## What to check

1. Check the indicated media location and the reported open error.

## Recommended action

Check the indicated media location and the reported open error.
