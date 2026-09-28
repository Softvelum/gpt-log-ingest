# Memory or DVR response range prepared

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-2ace1c054b40afe0

## Log pattern

```text
requested range='%s' size=%llu response header:
%s
```

## Parameters

- Argument 1 — Requested range: Text of the client Range field, or empty when absent.
- Argument 2 — Response bytes: Unsigned 64-bit selected response length in bytes, with the HEAD difference described above.
- Argument 3 — Response headers: Text of the assembled HTTP response headers.

## Log examples

Illustrative examples; these are not captured server logs.

```text
requested range='bytes=0-65535' size=65536 response header:
HTTP/1.1 206 Partial Content
Server: Nimble/3.7.0
Date: Mon, 28 Sep 2026 12:00:00 GMT
Content-Type: video/mp4
Content-Length: 65536
Accept-Ranges: bytes
Cache-Control: no-cache
Content-Range: bytes 0-65535/65536
Connection: close


```

## Short description

Memory or DVR response range prepared.

## Trigger condition

A memory-backed body or file-backed DVR segment has passed range validation and its HTTP headers are assembled.

## Explanation

The session switches to client output. For memory responses the logged size is zero for HEAD; the DVR branch logs the selected range length even for HEAD.

## Possible causes

- A memory-backed body or file-backed DVR segment has passed range validation and its HTTP headers are assembled.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
