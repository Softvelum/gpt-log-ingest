# RTMP TLS write reports an unrecoverable error

Product: Nimble Streamer
Component: RTMP connection transport
Logging function: log_e
Log level: error
Documentation ID: log_e-b791e8db0ae9e5f6

## Log pattern

```text
failed to send ssl buffer to %s s=%d
```

## Parameters

- Argument 1 — Peer label: Diagnostic name assigned to the TLS peer socket.
- Argument 2 — Socket: Operating-system socket descriptor for this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send ssl buffer to rtmp_camera s=42
```

## Short description

RTMP TLS write reports an unrecoverable error.

## Trigger condition

A TLS send returns an error without requesting a retry for read or write readiness.

## Explanation

The send is reported as failed to the caller. Normal TLS readiness waits use another path and do not emit this message.

## Possible causes

- A TLS send returns an error without requesting a retry for read or write readiness.

## What to check

1. Check the TLS peer label, socket, and accompanying TLS diagnostics.

## Recommended action

Resolve the TLS or transport failure before retrying the connection.
