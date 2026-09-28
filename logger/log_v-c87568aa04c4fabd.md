# Cache has no room for an item

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c87568aa04c4fabd

## Log pattern

```text
DataCache: not enough space for item key='%s' size=%llu ram_cache_size=%llu ram_cache_size_limit=%llu disk_cache_size=%llu disk_cache_size_limit=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — requested bytes: Bytes required by this allocation; fragmented items initially request one fragment.
- Argument 3 — RAM bytes: Total cache capacity currently allocated in memory, in bytes.
- Argument 4 — RAM limit: Configured RAM cache limit in bytes.
- Argument 5 — disk bytes: Total cache capacity currently accounted for on disk, in bytes.
- Argument 6 — disk limit: Configured disk cache limit in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DataCache: not enough space for item key='movie' size=4096 ram_cache_size=8192 ram_cache_size_limit=8192 disk_cache_size=65536 disk_cache_size_limit=65536
```

## Short description

Cache has no room for an item.

## Trigger condition

The selected memory and disk allocation paths cannot satisfy the item size, including any attempted removal of eligible old entries.

## Explanation

No data storage is allocated. A cache-space flag is set for the next statistics read and allocation returns failure.

## Possible causes

- Cache limits leave insufficient space and enough eligible unused data cannot be removed.

## What to check

1. Compare requested bytes, occupied capacity, limits, and minimum VOD storage time.

## Recommended action

Provide sufficient cache capacity or reduce the amount of data that must remain cached.
