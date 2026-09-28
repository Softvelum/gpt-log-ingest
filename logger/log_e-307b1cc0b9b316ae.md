# Cache item released without a reference

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-307b1cc0b9b316ae

## Log pattern

```text
CacheItem::Release failed for item key='%s' state=%d ref_count=%d
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — state: Numeric cache state: 0 empty, 1 downloading, 2 fragmented, 3 ready, or 4 marked for removal.
- Argument 3 — references: Current number of users holding the item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
CacheItem::Release failed for item key='movie' state=3 ref_count=0
```

## Short description

Cache item released without a reference.

## Trigger condition

An item release sees a nonpositive reference count.

## Explanation

The cache throws a state error before reducing the count or releasing storage.

## Possible causes

- The item was released without a matching held reference.

## What to check

1. Correlate release and removal messages for the cache key.

## Recommended action

Investigate the cache ownership sequence.
