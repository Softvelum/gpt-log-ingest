# File block read for a plain socket transfer

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-ab9bb4c1cf6e3c67

## Log pattern

```text
read %u bytes from fd=%d offset=%lld
```

## Parameters

- Argument 1 — Read bytes: Requested and successfully read block length in bytes, supplied as the platform size type.
- Argument 2 — File descriptor: Integer descriptor of the source file.
- Argument 3 — File offset: Requested byte offset within the source file.

The read size is passed with the platform size type to an unsigned-int conversion; compatibility depends on the Windows build.

## Log examples

Illustrative examples; these are not captured server logs.

```text
read 4096 bytes from fd=10 offset=8192
```

## Short description

File block read for a plain socket transfer.

## Trigger condition

On Windows, the source file read returns exactly the requested byte count.

## Explanation

The block is sent to the socket next. The file offset is advanced by the number of bytes actually sent, not by this logged read count.
