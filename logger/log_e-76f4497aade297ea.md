# File seek failed before a plain socket transfer

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_e
Log level: error
Documentation ID: log_e-76f4497aade297ea

## Log pattern

```text
failed to seek to offset=%lld for fd=%d
```

## Parameters

- Argument 1 — File offset: Requested byte offset within the source file.
- Argument 2 — File descriptor: Integer descriptor of the source file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to seek to offset=4096 for fd=10
```

## Short description

File seek failed before a plain socket transfer.

## Trigger condition

On Windows, seeking the source file for a non-TLS send does not return the requested position.

## Explanation

The file-send helper returns failure without reading or sending the requested block.

## Possible causes

- The file could not be positioned at the requested byte offset.

## What to check

1. Check the requested offset and source file.
