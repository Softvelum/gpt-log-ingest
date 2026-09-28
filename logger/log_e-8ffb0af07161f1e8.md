# Connection protocol buffer allocation failed

Product: Nimble Streamer
Component: Icecast, MPEG-TS HTTP input, and RTMP connections
Logging function: log_e
Log level: error
Documentation ID: log_e-8ffb0af07161f1e8

## Log pattern

```text
cannot allocate buffer to client: %d
```

## Parameters

- Argument 1 — socket: Socket descriptor for the affected connection, not a buffer size.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot allocate buffer to client: 42
```

## Short description

Connection protocol buffer allocation failed.

## Trigger condition

A connection cannot reserve or write a buffer needed for the next protocol operation.

## Explanation

The message covers Icecast input-buffer reservation, HTTP 200 or 401 response construction, origin GET storage, and post-send receive-buffer reservation; MPEG-TS origin GET storage; and RTMP handshake, connect, create-stream, subscribe, play, publish, and delete-stream buffers. Every occurrence requests connection shutdown and returns, with false returned where the operation has a boolean result. The wording to client also appears on origin pull paths and for incoming buffers. The descriptor may be -1 when preparation occurs before connection.

## Possible causes

- A required buffer reservation or write failed, for example because capacity could not be obtained.

## What to check

1. Identify the protocol operation from nearby messages and check request size, buffer limits, and memory diagnostics.

## Recommended action

Resolve the indicated allocation or oversized-protocol-data problem before retrying the connection.
