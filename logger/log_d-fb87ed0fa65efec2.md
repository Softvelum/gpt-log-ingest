# Saved cache item registered

Product: Nimble Streamer
Component: Media data cache
Logging function: log_d
Log level: debug
Documentation ID: log_d-fb87ed0fa65efec2

## Log pattern

```text
DataCache loaded item key='%s' file='%s'
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — file: Filesystem path of the cache data or metadata file being handled.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DataCache loaded item key='movie' file='/var/cache/nimble/movie'
```

## Short description

Saved cache item registered.

## Trigger condition

A saved item passes metadata handling and is inserted into the in-memory cache map.

## Explanation

Its capacity is added to the disk cache total; fragmented items also reconstruct their available fragment records.
