# Client request read starting

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-756ce43b66d3e5ac

## Log pattern

```text
kEventIn for s=%d reading to 0x%lld[%d]
```

## Parameters

- Argument 1 — Client socket: Integer identifying the local client socket.
- Argument 2 — Buffer address: Pointer cast to a signed 64-bit integer and printed in decimal despite the literal 0x prefix.
- Argument 3 — Buffered request bytes: Integer offset in bytes where the next read will append data.

## Log examples

Illustrative examples; these are not captured server logs.

```text
kEventIn for s=18 reading to 0x4660[256]
```

## Short description

Client request read starting.

## Trigger condition

The expected client socket is ready for another request read.

## Explanation

The request-buffer address and current offset are recorded before capacity checking and reading.

## Possible causes

- The expected client socket is ready for another request read.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
