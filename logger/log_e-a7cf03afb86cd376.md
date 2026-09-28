# File read was incomplete before a plain socket transfer

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_e
Log level: error
Documentation ID: log_e-a7cf03afb86cd376

## Log pattern

```text
failed to read %u bytes from fd=%d offset=%lld read=%d errno=%d
```

## Parameters

- Argument 1 — Requested bytes: Requested block length, at most 128 KiB, supplied as the platform size type.
- Argument 2 — File descriptor: Integer descriptor of the source file.
- Argument 3 — File offset: Requested byte offset within the source file.
- Argument 4 — Read result: Actual signed byte count from the file read.
- Argument 5 — System error: Integer operating-system socket error value.

The requested size is passed with the platform size type to an unsigned-int conversion; compatibility depends on the Windows build.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read 4096 bytes from fd=10 offset=8192 read=0 errno=0
```

## Short description

File read was incomplete before a plain socket transfer.

## Trigger condition

On Windows, the file read returns a count different from the requested block size.

## Explanation

The helper returns failure without sending that block. A short read as well as a negative read result reaches this branch.

## Possible causes

- The file read returned fewer or otherwise different bytes than requested.

## What to check

1. Compare the requested count, actual count and file offset.
