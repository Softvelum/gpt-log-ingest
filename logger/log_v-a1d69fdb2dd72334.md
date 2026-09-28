# Cache lookup found an item

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-a1d69fdb2dd72334

## Log pattern

```text
found key='%s' st=%u sz=%llu ref=%u
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — state: Numeric cache state: 0 empty, 1 downloading, 2 fragmented, 3 ready, or 4 marked for removal.
- Argument 3 — size: Stored data size in bytes.
- Argument 4 — references: Current number of users holding the item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
found key='movie' st=3 sz=4096 ref=1
```

## Short description

Cache lookup found an item.

## Trigger condition

A matching item exists and is not marked for removal.

## Explanation

The cache increases the reference count before this message. Reopening disk storage may still fail afterward, so this is not proof that the request received usable data.
