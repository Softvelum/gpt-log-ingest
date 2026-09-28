# Media response headers prepared

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-eb519b1c1bac2361

## Log pattern

```text
client s=%d response:
%s<...>
```

## Parameters

- Argument 1 — Client socket: Integer identifying the local client socket.
- Argument 2 — Response headers: Text of the assembled HTTP headers; the media body is not printed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
client s=18 response:
HTTP/1.1 200 OK
Date: Mon, 28 Sep 2026 12:00:00 GMT
Content-Type: video/mp2t
Server: Nimble/3.7.0
Content-Length: 65536
Cache-Control: no-cache
Connection: close

<...>
```

## Short description

Media response headers prepared.

## Trigger condition

A received origin chunk, streaming origin response, or cached body has passed response-header preparation.

## Explanation

The headers are logged before client output; the literal <...> stands for media bytes omitted from this diagnostic.

## Possible causes

- A received origin chunk, streaming origin response, or cached body has passed response-header preparation.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
