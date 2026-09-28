# Cache parent directory cannot be inspected

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-d09c73bd4cb2710f

## Log pattern

```text
DataCache::DeleteFileAndMeta failed to iterate '%s' error=%d message='%s'
```

## Parameters

- Argument 1 — directory: Parent directory being checked or removed.
- Argument 2 — error: Filesystem error number.
- Argument 3 — message: Filesystem error description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DataCache::DeleteFileAndMeta failed to iterate '/var/cache/nimble/app' error=13 message='Permission denied'
```

## Short description

Cache parent directory cannot be inspected.

## Trigger condition

The filesystem reports an error while iterating a parent directory.

## Explanation

Further upward directory cleanup stops. Cache data and metadata deletion was already attempted before this parent-directory pass.

## Possible causes

- The filesystem operation failed for the parent directory.

## What to check

1. Check directory access and the reported filesystem error.

## Recommended action

Correct the directory problem if empty cache directories need to be removed.
