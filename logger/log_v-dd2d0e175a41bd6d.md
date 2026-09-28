# Short-lived item moved to the back

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-dd2d0e175a41bd6d

## Log pattern

```text
set last short lived item='%s'
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
set last short lived item='movie'
```

## Short description

Short-lived item moved to the back.

## Trigger condition

The cache places a short-lived item at the end of its dedicated list.

## Explanation

Its former position is removed and the next cleanup time is recalculated from the item now at the front.
