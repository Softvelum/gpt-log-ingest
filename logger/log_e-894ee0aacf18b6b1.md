# Cache file resize failed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-894ee0aacf18b6b1

## Log pattern

```text
ftruncate fd=%d size=%llu failed
```

## Parameters

- Argument 1 — descriptor: Open file descriptor used for the operation.
- Argument 2 — requested bytes: Requested file and mapping length in bytes.

The shared helper passes a platform-sized byte count to a 64-bit unsigned conversion; format compatibility depends on the build.

## Log examples

A portable rendered example is not determined because one occurrence uses a platform-dependent argument width.

## Short description

Cache file resize failed.

## Trigger condition

Resizing a newly opened non-Windows file to the requested mapping length fails.

## Explanation

Both the direct cache path and the shared file-mapping helper continue to attempt a memory mapping after this error. The message itself includes no operating system error number.

## Possible causes

- The operating system rejected the file resize.

## What to check

1. Check available storage and whether the file can be resized; inspect the next mapping result.

## Recommended action

Correct the resize failure before relying on the allocated file.
