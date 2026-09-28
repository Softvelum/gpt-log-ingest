# TLS shutdown encountered a broken pipe

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-ef4ae57a285f081a

## Log pattern

```text
failed to SSL_shutdown %s s=%d (SSL_errno=%d errno=%d)
```

## Parameters

- Argument 1 — Peer role: Text name identifying the socket role, such as client or origin.
- Argument 2 — Socket: Integer socket descriptor.
- Argument 3 — TLS result: Integer TLS error category returned for the failed operation.
- Argument 4 — System error: Integer operating-system socket error value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to SSL_shutdown client s=12 (SSL_errno=5 errno=32)
```

## Short description

TLS shutdown encountered a broken pipe.

## Trigger condition

TLS shutdown returns a negative result classified as a system-call error, with the system error indicating a broken pipe.

## Explanation

The condition is logged at debug level and shutdown returns failure.

## Possible causes

- The TLS shutdown operation encountered a broken pipe.

## What to check

1. Inspect the peer closure sequence for this socket.
