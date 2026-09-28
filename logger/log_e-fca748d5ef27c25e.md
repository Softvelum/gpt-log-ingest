# Cache metadata write was incomplete

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-fca748d5ef27c25e

## Log pattern

```text
failed to write metadata file='%s' fd=%d wrote %lld of %lld bytes (errno=%d)
```

## Parameters

- Argument 1 — file: Filesystem path of the cache data or metadata file being handled.
- Argument 2 — descriptor: Open file descriptor used for the operation.
- Argument 3 — written: Write result in bytes, or a negative result on failure.
- Argument 4 — expected: Serialized metadata length in bytes.
- Argument 5 — error: Operating system error number reported for the failed operation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to write metadata file='/var/cache/nimble/movie.meta' fd=12 wrote -1 of 200 bytes (errno=28)
```

## Short description

Cache metadata write was incomplete.

## Trigger condition

The metadata write returns a byte count different from the serialized metadata length.

## Explanation

The file is closed and metadata saving reports failure; the caller handles the affected cache item.

## Possible causes

- The write failed or wrote only part of the metadata.

## What to check

1. Compare the write result with the expected length and check storage capacity and errors.

## Recommended action

Resolve the write failure before relying on disk cache persistence.
