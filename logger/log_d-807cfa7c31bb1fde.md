# Origin body read into a full buffer

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-807cfa7c31bb1fde

## Log pattern

```text
received %ld (total %llu) body bytes from origin s=%d
```

## Parameters

- Argument 1 — Read bytes: Signed native-size bytes returned by the read, printed as long.
- Argument 2 — Displayed total: Unsigned 64-bit expected response-body size plus this read length; not a progress total.
- Argument 3 — Origin socket: Integer identifying the local origin socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
received 8192 (total 73728) body bytes from origin s=17
```

## Short description

Origin body read into a full buffer.

## Trigger condition

A positive origin-body read completes in non-circular buffer mode.

## Explanation

The bytes become available for client output. The field labeled total is the expected total body size plus this read, not the cumulative received count.

## Possible causes

- A positive origin-body read completes in non-circular buffer mode.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
