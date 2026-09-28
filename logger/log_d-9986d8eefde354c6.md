# Additional TLS send error

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-9986d8eefde354c6

## Log pattern

```text
E: ssl=%p s=%d (ssl_e=%lu e='%s')
```

## Parameters

- Argument 1 — TLS session: TLS session address used for diagnostic correlation.
- Argument 2 — Socket: Integer socket descriptor.
- Argument 3 — TLS error code: Unsigned long TLS library error code.
- Argument 4 — TLS detail: Text decoded from the next queued TLS library error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
E: ssl=0x1000 s=12 (ssl_e=1 e='TLS error')
```

## Short description

Additional TLS send error.

## Trigger condition

After a failed TLS send, debug logging is enabled and another error remains in the TLS error queue.

## Explanation

One message is produced for each remaining error until the queue is empty. It supplements the preceding TLS send failure.

## Possible causes

- A failed TLS send left additional library error records.

## What to check

1. Read this detail together with the preceding send failure for the same TLS session.
