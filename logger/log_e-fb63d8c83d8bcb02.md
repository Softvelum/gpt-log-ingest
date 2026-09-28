# TLS synchronization request write failed

Product: Nimble Streamer
Component: Panel synchronization and proxy transport
Logging function: log_e
Log level: error
Documentation ID: log_e-fb63d8c83d8bcb02

## Log pattern

```text
SSL_write() failed with error wresult=%d ssl_error=%d errno=%d error='%s'
```

## Parameters

- Argument 1 — Write result: TLS write return value, signed integer.
- Argument 2 — TLS error: TLS error classification derived from this write result, integer.
- Argument 3 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.
- Argument 4 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
SSL_write() failed with error wresult=-1 ssl_error=1 errno=5 error='operation failed'
```

## Short description

TLS synchronization request write failed.

## Trigger condition

The TLS write operation returns zero or a negative result.

## Explanation

TLS session and context are freed and synchronization reports a TLS send failure.

## Possible causes

- TLS could not send the request.

## What to check

1. Check the write result, TLS error classification, operating system error, and queued TLS diagnostic.

## Recommended action

Resolve the specific TLS or transport failure.
