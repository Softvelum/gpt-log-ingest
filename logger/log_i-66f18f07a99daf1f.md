# Bus-signal recovery checks generated output state

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_i
Log level: info
Documentation ID: log_i-66f18f07a99daf1f

## Log pattern

```text
cache_item key='%s' state=%d
```

## Parameters

- Argument 1 — Output cache key: Text cache key reserved for the requested generated output.
- Argument 2 — Cache state: Integer cache state: 0 newly reserved, 1 receiving data, 2 fragmented, 3 ready, or 4 marked for removal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cache_item key='/vod/movie.mp4/n_0_0_0.ts' state=1
```

## Short description

Bus-signal recovery checks generated output state.

## Trigger condition

Bus-signal recovery still holds the requested output cache item.

## Explanation

Newly reserved or downloading output is canceled, and the held item reference is released. Other states are released without cancellation.

## Possible causes

- Bus-signal recovery still holds the requested output cache item.

## What to check

1. Use this key to find the playback request affected by the earlier mapped-file fault.

## Recommended action

Use this key to find the playback request affected by the earlier mapped-file fault.
