# Cached item became ready

Product: Nimble Streamer
Component: Media data cache
Logging function: log_d
Log level: debug
Documentation ID: log_d-37e1c981d54ddcad

## Log pattern

```text
key='%s' ready ct='%s' sz=%llu cap=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — content type: Content type stored with the item.
- Argument 3 — size: Stored data size in bytes.
- Argument 4 — capacity: Allocated capacity of the item in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
key='movie' ready ct='video/mp4' sz=4096 cap=4096
```

## Short description

Cached item became ready.

## Trigger condition

The downloading-to-ready transition succeeds.

## Explanation

The complete item can be served; observers are notified and the item moves to the most recently used end of its cache category.
