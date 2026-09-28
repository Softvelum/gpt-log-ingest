# Cache byte range availability requested

Product: Nimble Streamer
Component: Media data cache
Logging function: log_d
Log level: debug
Documentation ID: log_d-8aebee2d7af2de79

## Log pattern

```text
SetFragmentAvailable key='%s' from=%llu to=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — from: First byte offset in the range.
- Argument 3 — to: Exclusive ending byte offset.

## Log examples

Illustrative examples; these are not captured server logs.

```text
SetFragmentAvailable key='movie' from=0 to=1048576
```

## Short description

Cache byte range availability requested.

## Trigger condition

A completed byte range is submitted before alignment and range validation.

## Explanation

This trace records the requested range; it does not prove that the fragment became available. Invalid alignment or length subsequently raises a cache error.
