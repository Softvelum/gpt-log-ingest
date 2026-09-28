# TLS shutdown needs another readiness event

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-ed8fd44ce53d8000

## Log pattern

```text
SSL_shutdown s=%d ret=%d errno=%d
```

## Parameters

- Argument 1 — Socket: Integer socket descriptor.
- Argument 2 — Return value: Negative TLS shutdown result.
- Argument 3 — TLS result: Integer TLS error category returned for the failed operation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
SSL_shutdown s=12 ret=-1 errno=2
```

## Short description

TLS shutdown needs another readiness event.

## Trigger condition

TLS shutdown returns a negative result requesting read or write readiness.

## Explanation

The helper saves that TLS result and returns false. The message represents a pending TLS shutdown step, not necessarily an unrecoverable TLS failure.
