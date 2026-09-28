# File positioned for a plain socket transfer

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-c5e9c6789e513bda

## Log pattern

```text
lseek to offset=%lld for fd=%d
```

## Parameters

- Argument 1 — File offset: Requested byte offset within the source file.
- Argument 2 — File descriptor: Integer descriptor of the source file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
lseek to offset=4096 for fd=10
```

## Short description

File positioned for a plain socket transfer.

## Trigger condition

On Windows, seeking the source file for a non-TLS send returns the requested position.

## Explanation

A block of up to 128 KiB is read and then sent to the socket.
