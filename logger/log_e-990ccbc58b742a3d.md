# Cache cancellation lost a state race

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-990ccbc58b742a3d

## Log pattern

```text
CacheItem::SetCanceled race for item key='%s' state=%d
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — state: Numeric cache state: 0 empty, 1 downloading, 2 fragmented, 3 ready, or 4 marked for removal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
CacheItem::SetCanceled race for item key='movie' state=1
```

## Short description

Cache cancellation lost a state race.

## Trigger condition

The item state changes after cancellation reads it and before its atomic replacement.

## Explanation

The cancellation reason has already been stored, but the removal transition failed and a state error is thrown. The printed state is the earlier state, not the new competing value.

## Possible causes

- An internal cache state transition was requested at an invalid time.

## What to check

1. Compare nearby messages for the same cache key and its state changes.

## Recommended action

Use the surrounding cache lifecycle messages to identify the conflicting operation.
