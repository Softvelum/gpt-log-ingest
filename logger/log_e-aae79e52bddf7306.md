# Cache directory creation failed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-aae79e52bddf7306

## Log pattern

```text
create_directories is failed to create dir='%s' (error=%d)
```

## Parameters

- Argument 1 — directory: Directory requested for cache storage.
- Argument 2 — error: Filesystem library error number from directory creation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
create_directories is failed to create dir='/var/cache/nimble/app' (error=13)
```

## Short description

Cache directory creation failed.

## Trigger condition

Creating a missing fragment directory or parent directory reports failure.

## Explanation

The error is logged, but allocation continues to the later file-creation attempt, which may also fail.

## Possible causes

- The directory could not be created.

## What to check

1. Check parent directory access and the reported filesystem error.

## Recommended action

Correct directory access or storage availability.
