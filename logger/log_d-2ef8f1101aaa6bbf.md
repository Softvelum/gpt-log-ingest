# Empty cache item reserved

Product: Nimble Streamer
Component: Media data cache
Logging function: log_d
Log level: debug
Documentation ID: log_d-2ef8f1101aaa6bbf

## Log pattern

```text
reserved key='%s' sz=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — total cache bytes: Current combined RAM and disk cache capacity in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
reserved key='movie' sz=8192
```

## Short description

Empty cache item reserved.

## Trigger condition

A new empty cache object is inserted for the key.

## Explanation

The item can now receive storage and content. The printed size is total RAM plus disk cache capacity, not the size of this empty item.
