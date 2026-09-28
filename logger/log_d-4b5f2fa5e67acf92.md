# Remote subtitle source cannot be prepared

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_d
Log level: debug
Documentation ID: log_d-4b5f2fa5e67acf92

## Log pattern

```text
failed to get subtitles cache item for [%s]
```

## Parameters

- Argument 1 — Remote subtitle key: Text cache key of the requested SRT, WebVTT, or TTML source.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get subtitles cache item for [/vod/movie.srt]
```

## Short description

Remote subtitle source cannot be prepared.

## Trigger condition

Preparing the cached subtitle source fails: the extension may be unsupported, the item may be missing or non-fragmented, fragment zero may be unavailable, or its backing file may fail to load.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- Preparing the cached subtitle source fails: the extension may be unsupported, the item may be missing or non-fragmented, fragment zero may be unavailable, or its backing file may fail to load.

## What to check

1. Check the subtitle extension, fragmented cache state, first-fragment availability, and any preceding file-read error.

## Recommended action

Check the subtitle extension, fragmented cache state, first-fragment availability, and any preceding file-read error.
