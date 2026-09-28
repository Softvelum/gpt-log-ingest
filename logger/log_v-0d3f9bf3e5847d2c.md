# Cache invalidation pass started

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-0d3f9bf3e5847d2c

## Log pattern

```text
start invalidating cache ram=%llu disk=%llu items=%llu postponed=%llu time=%lld
```

## Parameters

- Argument 1 — RAM bytes: Total cache capacity currently allocated in memory, in bytes.
- Argument 2 — disk bytes: Total cache capacity currently accounted for on disk, in bytes.
- Argument 3 — items: Number of items currently registered by cache key.
- Argument 4 — postponed: Number of objects awaiting deferred deletion.
- Argument 5 — time: Current cache clock time in seconds.

The item count uses a platform-sized container count with a 64-bit unsigned conversion.

## Log examples

No portable example is provided for the platform-dependent item-count argument.

## Short description

Cache invalidation pass started.

## Trigger condition

Periodic cache cleanup acquires the cache lock and records its current totals.

## Explanation

The pass first checks deferred deletions when starting a fresh traversal, then removes eligible unused expired items and fragments. Work may be split over several passes.
