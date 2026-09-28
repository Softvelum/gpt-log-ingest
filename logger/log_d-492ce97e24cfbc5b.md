# TLS socket send failed

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-492ce97e24cfbc5b

## Log pattern

```text
failed to send to %s ssl=%p s=%d len=%u (errno=%d e='%s')
```

## Parameters

- Argument 1 — Peer role: Text name identifying the socket role, such as client or origin.
- Argument 2 — TLS session: TLS session address used for diagnostic correlation.
- Argument 3 — Socket: Integer socket descriptor.
- Argument 4 — Requested bytes: Original requested byte length, supplied as the platform size type.
- Argument 5 — TLS result: Integer TLS error category returned for the failed operation.
- Argument 6 — TLS detail: Text decoded from the next queued TLS library error.

The byte length is passed as the platform size type, while its conversion expects unsigned int. Those types need not match.

## Log examples

A dependable example is omitted because the byte-length argument can be wider than its conversion.

## Short description

TLS socket send failed.

## Trigger condition

A TLS write returns a negative result other than a read/write-readiness retry, with debug logging enabled.

## Explanation

The failure is reported, followed by any remaining queued TLS errors. The negative write result is returned to the caller.

## Possible causes

- TLS could not complete the write and did not request a readiness retry.

## What to check

1. Inspect the primary TLS error and subsequent queued TLS-error messages.
