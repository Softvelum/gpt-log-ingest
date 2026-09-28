# DVR timeline transport failed

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-dcc6f0a677ffef2d

## Log pattern

```text
failed to send DVR timeline to WMSPanel status=%d:
%s
```

## Parameters

- Argument 1 — sync\_status: Transport result number: 0 success, 1 send failure, 2 receive failure, 3 invalid HTTP response, 4 TLS context creation failure, 5 TLS object creation failure, 6 TLS socket assignment failure, 7 partial TLS send, 8 TLS request-send failure, 9 TLS read failure, 10 internal error.
- Argument 2 — request\_json: Complete DVR timeline request body.

## Log examples

No shortened timeline body is used as an example because this call prints the complete nested DVR report.

## Short description

DVR timeline transport failed.

## Trigger condition

A connected DVR timeline request returns a non-success local transport status.

## Explanation

The failure and complete request JSON are logged and the socket is shut down. This helper performs no immediate retry.

## Possible causes

- The request transport returned a send, receive, HTTP, TLS, or internal failure.

## What to check

1. Check the status code and preceding transport diagnostics.

## Recommended action

Resolve the indicated connection or response failure.
