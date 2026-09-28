# Plain socket shutdown completed

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-8db0ab5da0ec6af8

## Log pattern

```text
shutdown %s s=%d
```

## Parameters

- Argument 1 — Peer role: Text name identifying the socket role, such as client or origin.
- Argument 2 — Socket: Integer socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
shutdown client s=12
```

## Short description

Plain socket shutdown completed.

## Trigger condition

Shutting down both directions of a non-TLS socket succeeds.

## Explanation

The helper returns success; descriptor closure is a separate operation.
