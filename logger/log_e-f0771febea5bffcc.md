# Cache file close failed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-f0771febea5bffcc

## Log pattern

```text
close failed key='%s' fd=%d errno=%d
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — descriptor: Open file descriptor used for the operation.
- Argument 3 — error: Operating system error number reported for the failed operation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
close failed key='movie' fd=12 errno=9
```

## Short description

Cache file close failed.

## Trigger condition

Closing a non-Windows cache file descriptor fails during unmapping.

## Explanation

The descriptor is marked closed in the cache object despite the operating system error.

## Possible causes

- The operating system rejected the indicated cleanup operation.

## What to check

1. Use the error number and preceding mapping lifecycle messages for the key.

## Recommended action

Investigate the failed file cleanup.
