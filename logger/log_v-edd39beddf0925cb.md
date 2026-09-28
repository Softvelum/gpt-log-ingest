# Cache cleanup reached a retained item boundary

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-edd39beddf0925cb

## Log pattern

```text
s i=%d key='%s' st=%d ref=%u l=%d f=%d
```

## Parameters

- Argument 1 — iteration: Number of entries already examined in the current cleanup pass.
- Argument 2 — key: Cache lookup key identifying the stored item.
- Argument 3 — state: Numeric cache state: 0 empty, 1 downloading, 2 fragmented, 3 ready, or 4 marked for removal.
- Argument 4 — references: Current number of users holding the item.
- Argument 5 — live: 1 when the item has a positive timeout, otherwise 0.
- Argument 6 — file backed: 1 for a normal file-backed item, otherwise 0.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s i=2 key='movie' st=1 ref=1 l=0 f=1
```

## Short description

Cache cleanup reached a retained item boundary.

## Trigger condition

A nonremovable item enters the branch that skips to VOD entries or ends the traversal.

## Explanation

For an item without a positive timeout the traversal stops at this position. The same branch also contains a jump to the VOD boundary for live items; the outer work limit constrains when that branch can be reached.
