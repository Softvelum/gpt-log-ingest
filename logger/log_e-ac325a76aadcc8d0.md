# Fragmented cache transition rejected

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-ac325a76aadcc8d0

## Log pattern

```text
SetFragmented failed for item key='%s' state=%d
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — state: Numeric cache state: 0 empty, 1 downloading, 2 fragmented, 3 ready, or 4 marked for removal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
SetFragmented failed for item key='movie' state=4
```

## Short description

Fragmented cache transition rejected.

## Trigger condition

An item cannot move atomically from downloading to fragmented.

## Explanation

The cache throws a state error and does not notify waiting users that fragments are available.

## Possible causes

- An internal cache state transition was requested at an invalid time.

## What to check

1. Compare nearby messages for the same cache key and its state changes.

## Recommended action

Use the surrounding cache lifecycle messages to identify the conflicting operation.
