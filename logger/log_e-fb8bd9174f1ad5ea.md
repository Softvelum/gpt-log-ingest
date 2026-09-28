# Cache fragment already available

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-fb8bd9174f1ad5ea

## Log pattern

```text
item key='%s' fragment=%u is already available
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — fragment: Zero-based fragment index.

## Log examples

Illustrative examples; these are not captured server logs.

```text
item key='movie' fragment=0 is already available
```

## Short description

Cache fragment already available.

## Trigger condition

A request to mark a fragment available finds its availability bit already set.

## Explanation

The cache returns without changing the bit or sending another update notification.

## Possible causes

- The same fragment was marked available again.

## What to check

1. Check earlier completion messages for this key and fragment.

## Recommended action

Investigate duplicate fragment completion.
