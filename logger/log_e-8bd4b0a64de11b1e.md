# Remote VOD input cache item is missing

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-8bd4b0a64de11b1e

## Log pattern

```text
vod cache item not found key='%s'
```

## Parameters

- Argument 1 — Input cache key: Text identifying the missing remote media or subtitle source.

## Log examples

Illustrative examples; these are not captured server logs.

```text
vod cache item not found key='/vod/movie.mp4'
```

## Short description

Remote VOD input cache item is missing.

## Trigger condition

A requested remote MP4 input or a SMIL subtitle input cannot be retrieved from the cache.

## Explanation

Direct MP4 source preparation fails and its output is canceled. A missing SMIL subtitle input is skipped while manifest generation continues.

## Possible causes

- A requested remote MP4 input or a SMIL subtitle input cannot be retrieved from the cache.

## What to check

1. Check whether the source download failed or the item was removed before packaging; a missing subtitle entry can explain an incomplete manifest.

## Recommended action

Check whether the source download failed or the item was removed before packaging; a missing subtitle entry can explain an incomplete manifest.
