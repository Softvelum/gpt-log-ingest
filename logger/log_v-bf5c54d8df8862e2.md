# Cache item removed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-bf5c54d8df8862e2

## Log pattern

```text
removed key='%s' sz=%llu cap=%llu sl=%u ram=%llu disk=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — size: Stored data size in bytes.
- Argument 3 — capacity: Allocated capacity of the item in bytes.
- Argument 4 — short lived: 1 for a short-lived item, otherwise 0.
- Argument 5 — RAM bytes: Total cache capacity currently allocated in memory, in bytes.
- Argument 6 — disk bytes: Total cache capacity currently accounted for on disk, in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
removed key='movie' sz=4096 cap=4096 sl=0 ram=0 disk=8192
```

## Short description

Cache item removed.

## Trigger condition

An item is removed immediately rather than retained for deferred deletion.

## Explanation

The lookup and ordering entries are removed, associated storage cleanup is performed, and capacity accounting is updated before the object is deleted. File-removal success is not established by this message.
