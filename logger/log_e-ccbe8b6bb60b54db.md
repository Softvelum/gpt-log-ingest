# Remote audio source cache item is missing

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-ccbe8b6bb60b54db

## Log pattern

```text
item key='%s' is not found for transmuxing
```

## Parameters

- Argument 1 — Audio source key: Text cache key containing the requested remote MP3 or AAC source.

## Log examples

Illustrative examples; these are not captured server logs.

```text
item key='/vod/audio.mp3' is not found for transmuxing
```

## Short description

Remote audio source cache item is missing.

## Trigger condition

An MP3 or AAC transmuxing request names a remote source cache item that can no longer be retrieved.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- An MP3 or AAC transmuxing request names a remote source cache item that can no longer be retrieved.

## What to check

1. Check the remote audio download and cache removal events; the source item must remain available until audio packaging completes.

## Recommended action

Check the remote audio download and cache removal events; the source item must remain available until audio packaging completes.
