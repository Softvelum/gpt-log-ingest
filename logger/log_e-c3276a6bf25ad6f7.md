# Cache fragment unmapped with references remaining

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-c3276a6bf25ad6f7

## Log pattern

```text
unmapped item key='%s' fragment=%u ref_count=%u
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — fragment: Zero-based fragment index.
- Argument 3 — references: Fragment reference count at unmapping time.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unmapped item key='movie' fragment=0 ref_count=1
```

## Short description

Cache fragment unmapped with references remaining.

## Trigger condition

Bulk item cleanup unmaps a fragment whose recorded reference count is nonzero.

## Explanation

The fragment reference count is reset to zero. An unavailable fragment may also be removed, depending on the parent item state.

## Possible causes

- The parent item cleanup reached a fragment still recorded as referenced.

## What to check

1. Compare fragment use and release messages with the parent item cleanup.

## Recommended action

Investigate the fragment ownership mismatch.
