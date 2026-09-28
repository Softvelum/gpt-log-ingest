# Local file response range prepared

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-dbb8ee0273c3c5b0

## Log pattern

```text
requested range='%s' size=%llu resp header:
%s
```

## Parameters

- Argument 1 — Requested range: Text of the client Range field, or empty when absent.
- Argument 2 — Response body bytes: Unsigned 64-bit bytes selected for sending, zero for HEAD.
- Argument 3 — Response headers: Text of the assembled HTTP response headers.

## Log examples

Illustrative examples; these are not captured server logs.

```text
requested range='bytes=0-65535' size=65536 resp header:
HTTP/1.1 206 Partial Content
Server: Nimble/3.7.0
Date: Mon, 28 Sep 2026 12:00:00 GMT
Content-Type: video/mp4
Content-Length: 65536
Accept-Ranges: bytes
Cache-Control: no-cache
Content-Range: bytes 0-65535/131072
Connection: close


```

## Short description

Local file response range prepared.

## Trigger condition

A local file is open and its requested range and conditional-response checks have passed.

## Explanation

The response headers are logged and client output begins.

## Possible causes

- A local file is open and its requested range and conditional-response checks have passed.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
