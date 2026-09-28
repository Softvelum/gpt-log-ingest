# Playlist media format allocation failed

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-c6aa33c76dfb8a0c

## Log pattern

```text
failed to alloc avformat ctx, url=%s
```

## Parameters

- Argument 1 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to alloc avformat ctx, url=/media/program.mp4
```

## Short description

Playlist media format allocation failed.

## Trigger condition

Allocating the media format context for cached input fails.

## Explanation

Opening the media reader returns failure.

## Possible causes

- Allocating the media format context for cached input fails.

## What to check

1. Check available memory for the media reader.

## Recommended action

Check available memory for the media reader.
