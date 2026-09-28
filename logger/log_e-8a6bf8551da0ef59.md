# Cache content type set after the empty state

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-8a6bf8551da0ef59

## Log pattern

```text
CacheItem::SetContentType failed for item key='%s' state=%d
```

## Parameters

- Argument 1 — key: Cache lookup key.
- Argument 2 — state: State: 0 empty, 1 downloading, 2 fragmented, 3 ready, or 4 marked for removal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
CacheItem::SetContentType failed for item key='movie' state=3
```

## Short description

Cache content type set after the empty state.

## Trigger condition

A content-type assignment is requested when the cache item is not empty.

## Explanation

The assignment is rejected with a state error; the stored content type is not changed.

## Possible causes

- Content type was assigned after the initial empty item state.

## What to check

1. Compare the key with earlier allocation and state-change messages.

## Recommended action

Investigate the ordering of cache metadata updates.
