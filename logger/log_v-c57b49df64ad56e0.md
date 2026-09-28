# Short-lived item moved to the front

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c57b49df64ad56e0

## Log pattern

```text
set first short lived item='%s'
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
set first short lived item='movie'
```

## Short description

Short-lived item moved to the front.

## Trigger condition

The cache schedules a short-lived item at the front of its dedicated list.

## Explanation

Its former position is removed and the next cleanup time is recalculated from this item.
