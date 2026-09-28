# Remote audio source is not fully ready

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-7a2ce91f4d1b2b28

## Log pattern

```text
item key='%s' state=%d is not ready for transmuxing
```

## Parameters

- Argument 1 — Audio source key: Text cache key for the remote audio source.
- Argument 2 — Cache state: Integer cache state: 0 newly reserved, 1 receiving data, 2 fragmented, 3 ready, or 4 marked for removal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
item key='/vod/audio.mp3' state=1 is not ready for transmuxing
```

## Short description

Remote audio source is not fully ready.

## Trigger condition

The remote MP3 or AAC cache item exists but is not in the ready state.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- The remote MP3 or AAC cache item exists but is not in the ready state.

## What to check

1. Wait for the entire remote audio item to finish downloading; this path requires state 3 and does not process partial or fragmented input.

## Recommended action

Wait for the entire remote audio item to finish downloading; this path requires state 3 and does not process partial or fragmented input.
