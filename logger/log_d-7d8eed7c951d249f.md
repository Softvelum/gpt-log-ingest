# Remote fragmented-MP4 subtitle playlist is unsupported

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_d
Log level: debug
Documentation ID: log_d-7d8eed7c951d249f

## Log pattern

```text
invalid fmp4 subtitles list request for remote cache item [%s]
```

## Parameters

- Argument 1 — Remote subtitle key: Text cache key used as the subtitle source.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid fmp4 subtitles list request for remote cache item [/vod/movie.srt]
```

## Short description

Remote fragmented-MP4 subtitle playlist is unsupported.

## Trigger condition

A subtitle playlist request uses the fragmented-MP4 advertising form while its source is a remote cache item.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- A subtitle playlist request uses the fragmented-MP4 advertising form while its source is a remote cache item.

## What to check

1. Use a supported subtitle delivery form or local subtitle source; this path explicitly rejects the remote fragmented-MP4 subtitle-list combination.

## Recommended action

Use a supported subtitle delivery form or local subtitle source; this path explicitly rejects the remote fragmented-MP4 subtitle-list combination.
