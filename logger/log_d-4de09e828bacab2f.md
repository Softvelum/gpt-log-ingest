# TLS shutdown encountered a protocol error

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-4de09e828bacab2f

## Log pattern

```text
failed to SSL_shutdown %s s=%d (SSL_errno=%d e='%s')
```

## Parameters

- Argument 1 — Peer role: Text name identifying the socket role, such as client or origin.
- Argument 2 — Socket: Integer socket descriptor.
- Argument 3 — TLS result: Integer TLS error category returned for the failed operation.
- Argument 4 — TLS detail: Text decoded from the next queued TLS library error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to SSL_shutdown client s=12 (SSL_errno=1 e='TLS protocol error')
```

## Short description

TLS shutdown encountered a protocol error.

## Trigger condition

TLS shutdown returns a negative result that is neither a system-call error nor a request to wait for read/write readiness.

## Explanation

The TLS detail is reported and shutdown returns failure.

## Possible causes

- TLS shutdown failed without a readiness-retry result.

## What to check

1. Inspect the TLS category and detail.
