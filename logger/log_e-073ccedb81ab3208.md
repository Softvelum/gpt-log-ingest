# Buffered media-file read failed

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-073ccedb81ab3208

## Log pattern

```text
open_and_map_read_only_file read() failed file_name='%s' errno=%d, offset=%llu,read=%llu
```

## Parameters

- Argument 1 — File: Requested runtime data-file path.
- Argument 2 — Error: Operating-system error number.
- Argument 3 — Offset: Requested starting byte offset.
- Argument 4 — Read: Total bytes successfully read before the error.

The two unsigned 64-bit quantities use long-long conversions; exact typedef compatibility depends on the target platform.

## Log examples

Illustrative examples; these are not captured server logs.

```text
open_and_map_read_only_file read() failed file_name='/data/media/segment.m4s' errno=5, offset=4096,read=1024
```

## Short description

Buffered media-file read failed.

## Trigger condition

Reading the requested file region fails with an error other than an interrupted system call.

## Explanation

The partial buffer is not returned as a successful reader. The offset is the requested starting position and the read count is accumulated progress.

## Possible causes

- Reading the requested file region fails with an error other than an interrupted system call.

## What to check

1. Inspect the error, requested region, and bytes successfully read.
