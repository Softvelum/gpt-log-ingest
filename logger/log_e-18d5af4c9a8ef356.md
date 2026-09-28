# Cached file mapping failed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-18d5af4c9a8ef356

## Log pattern

```text
DataCache::RequestItem key='%s' file='%s' mmap failed errno=%d
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — file: Filesystem path of the cache data or metadata file being handled.
- Argument 3 — error: Operating system error number reported for the failed operation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DataCache::RequestItem key='movie' file='/var/cache/nimble/movie' mmap failed errno=12
```

## Short description

Cached file mapping failed.

## Trigger condition

A non-Windows cache lookup opened a file with the expected size but could not map it.

## Explanation

The descriptor is closed and the newly acquired reference is released. The lookup returns no data while leaving the cache entry recorded.

## Possible causes

- The operating system rejected the file mapping.

## What to check

1. Interpret the error number and inspect available address space and file access.

## Recommended action

Resolve the mapping failure before retrying the request.
