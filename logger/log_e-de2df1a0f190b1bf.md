# Video selection cannot produce the requested output

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-de2df1a0f190b1bf

## Log pattern

```text
file='%s' invalid video_index=%d
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Reported video selection: Integer selection recorded on the request. In fragmented-MP4 chunk and thumbnail branches, the failed lookup uses the index parsed from the chunk name, while this field still prints the request selection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
file='/media/movie.mp4' invalid video_index=9
```

## Short description

Video selection cannot produce the requested output.

## Trigger condition

A video-only playlist structure cannot be built for a master playlist, thumbnail, fragmented-MP4 chunk, or advertising-subtitle alignment.

## Explanation

The output is canceled, directly or through the subtitle helper caller.

## Possible causes

- A video-only playlist structure cannot be built for a master playlist, thumbnail, fragmented-MP4 chunk, or advertising-subtitle alignment.

## What to check

1. Check both the query track selection and the index embedded in the chunk name against the source video tracks.

## Recommended action

Check both the query track selection and the index embedded in the chunk name against the source video tracks.
