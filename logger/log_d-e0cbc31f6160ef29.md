# Cached audio playlist structure reused

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_d
Log level: debug
Documentation ID: log_d-e0cbc31f6160ef29

## Log pattern

```text
using cached playlist structure for f='%s' cached=%llu
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Media timestamp: Source modification time in epoch seconds; for remote MP4 cache input the stored source-cache timestamp may be used. This is not the last playlist access time.

The timestamp has type time_t but is printed with %llu; the illustrative positive value assumes a platform with a compatible 64-bit time representation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
using cached playlist structure for f='/media/movie.mp4' cached=1790596800
```

## Short description

Cached audio playlist structure reused.

## Trigger condition

The playlist-structure cache contains an entry matching the media name, modification timestamp, file size, and selected tracks.

## Explanation

The cached structure is reused to generate requested playlists or media without rebuilding it.

## Possible causes

- The playlist-structure cache contains an entry matching the media name, modification timestamp, file size, and selected tracks.

## What to check

1. Not applicable; this is normal processing progress. Follow the same source and output cache key if troubleshooting.

## Recommended action

Not applicable; this is normal processing progress. Follow the same source and output cache key if troubleshooting.
