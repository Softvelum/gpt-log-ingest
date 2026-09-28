# Socket receive would block

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-a9a30ce0b9730bd6

## Log pattern

```text
recv error=%d (EAGAIN | EWOULDBLOCK) on %s s=%d
```

## Parameters

- Argument 1 — System error: Integer socket error indicating a would-block condition.
- Argument 2 — Peer role: Text identifying the socket role.
- Argument 3 — Socket: Integer socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
recv error=11 (EAGAIN | EWOULDBLOCK) on client s=12
```

## Short description

Socket receive would block.

## Trigger condition

A non-TLS receive returns an error indicating that no data is available without waiting.

## Explanation

This condition is treated as temporary, not as a fatal receive error. Buffered reads report that they are awaiting more data.
