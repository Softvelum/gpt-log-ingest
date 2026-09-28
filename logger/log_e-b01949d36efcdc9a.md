# Playlist cached-media input allocation failed

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-b01949d36efcdc9a

## Log pattern

```text
failed to alloc avio ctx, url=%s
```

## Parameters

- Argument 1 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to alloc avio ctx, url=/media/program.mp4
```

## Short description

Playlist cached-media input allocation failed.

## Trigger condition

Creating a custom cached-media I/O context fails.

## Explanation

The allocated input buffer is released and opening returns failure.

## Possible causes

- Creating a custom cached-media I/O context fails.

## What to check

1. Check available memory and preceding allocation failures.

## Recommended action

Check available memory and preceding allocation failures.
