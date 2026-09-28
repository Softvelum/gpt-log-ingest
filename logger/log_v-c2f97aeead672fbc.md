# Postponed cache object deleted

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c2f97aeead672fbc

## Log pattern

```text
removed p key='%s' sz=%llu cap=%llu ram=%llu disk=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — size: Stored data size in bytes.
- Argument 3 — capacity: Allocated capacity of the item in bytes.
- Argument 4 — RAM bytes: Total cache capacity currently allocated in memory, in bytes.
- Argument 5 — disk bytes: Total cache capacity currently accounted for on disk, in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
removed p key='movie' sz=4096 cap=4096 ram=0 disk=8192
```

## Short description

Postponed cache object deleted.

## Trigger condition

A previously deferred item has no remaining references and its storage cleanup is performed.

## Explanation

This can happen during periodic cleanup or the final reference release. RAM or disk accounting has been reduced before the object is destroyed.
