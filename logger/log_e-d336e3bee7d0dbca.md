# Playlist cached-media input buffer allocation failed

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-d336e3bee7d0dbca

## Log pattern

```text
failed to alloc avio ctx buffer, url=%s
```

## Parameters

- Argument 1 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to alloc avio ctx buffer, url=/media/program.mp4
```

## Short description

Playlist cached-media input buffer allocation failed.

## Trigger condition

Allocating the 4 KiB input buffer for cached media fails.

## Explanation

Opening the media reader returns failure.

## Possible causes

- Allocating the 4 KiB input buffer for cached media fails.

## What to check

1. Check available memory before retrying cached-media playback.

## Recommended action

Check available memory before retrying cached-media playback.
