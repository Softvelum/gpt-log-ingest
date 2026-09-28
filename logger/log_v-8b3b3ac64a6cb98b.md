# Cache cleanup retained an unexpired live item

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-8b3b3ac64a6cb98b

## Log pattern

```text
f i=%d key='%s' st=%d ref=%u l=%d f=%d et=%lld to=%lld
```

## Parameters

- Argument 1 — iteration: Number of entries already examined in the current cleanup pass.
- Argument 2 — key: Cache lookup key identifying the stored item.
- Argument 3 — state: Numeric cache state: 0 empty, 1 downloading, 2 fragmented, 3 ready, or 4 marked for removal.
- Argument 4 — references: Current number of users holding the item.
- Argument 5 — live: 1 when the item has a positive timeout, otherwise 0.
- Argument 6 — file backed: 1 for a normal file-backed item, otherwise 0.
- Argument 7 — expiry: Stored expiration time in seconds.
- Argument 8 — timeout: Item lifetime in seconds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
f i=2 key='movie' st=3 ref=0 l=1 f=0 et=1700000045 to=45
```

## Short description

Cache cleanup retained an unexpired live item.

## Trigger condition

The current item is not marked, fragmented, or expired and has a positive timeout.

## Explanation

The cleanup traversal continues through live entries rather than stopping at this unexpired item.
