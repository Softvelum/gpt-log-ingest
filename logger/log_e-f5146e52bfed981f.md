# Cache allocation lacks a reserved item

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-f5146e52bfed981f

## Log pattern

```text
DataCache::AllocateItemData item key='%s' is not in the cache
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DataCache::AllocateItemData item key='movie' is not in the cache
```

## Short description

Cache allocation lacks a reserved item.

## Trigger condition

Data allocation is requested for a key absent from the cache map.

## Explanation

Allocation returns failure before assigning memory or disk storage.

## Possible causes

- No current cache reservation exists for the supplied item.

## What to check

1. Look for reservation and removal messages for this key.

## Recommended action

Investigate reservation lifetime before data allocation.
