# Writable file mapping failed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-7d0bcdfa29f653af

## Log pattern

```text
mmap failed fd=%d errno=%d
```

## Parameters

- Argument 1 — descriptor: Open file descriptor used for the operation.
- Argument 2 — error: Operating system error number reported for the failed operation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
mmap failed fd=12 errno=12
```

## Short description

Writable file mapping failed.

## Trigger condition

A non-Windows mapping of a newly opened or created file fails.

## Explanation

The descriptor is closed. The direct cache path also deletes the data and metadata files; the shared mapping helper returns failure without deleting the file.

## Possible causes

- The operating system rejected the writable shared mapping.

## What to check

1. Interpret the error and check file size and available address space.

## Recommended action

Resolve the mapping failure before retrying cache allocation.
