# TCP send buffer size could not be set

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_e
Log level: error
Documentation ID: log_e-90ddde47e9e1c336

## Log pattern

```text
setsockopt SO_SNDBUF failed for %s s=%d errno=%d
```

## Parameters

- Argument 1 — Peer role: Text name identifying the socket role, such as client or origin.
- Argument 2 — Socket: Integer socket descriptor.
- Argument 3 — System error: Integer operating-system socket error value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
setsockopt SO_SNDBUF failed for client s=12 errno=9
```

## Short description

TCP send buffer size could not be set.

## Trigger condition

Setting the socket send-buffer size returns failure.

## Explanation

The failure is logged and the helper returns without retrying or closing the socket.

## Possible causes

- The operating system rejected the requested send-buffer setting.

## What to check

1. Inspect the socket and reported system error.
