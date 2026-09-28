# Complete short HTTP response prepared

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-2f4cd9401b011eda

## Log pattern

```text
client s=%d response:
%s
```

## Parameters

- Argument 1 — Client socket: Integer identifying the local client socket.
- Argument 2 — Response buffer: Text of the complete prepared header buffer, including any embedded small body.

## Log examples

Illustrative examples; these are not captured server logs.

```text
client s=18 response:
HTTP/1.1 200 OK
Date: Mon, 28 Sep 2026 12:00:00 GMT
Server: Nimble/3.7.0
Content-Length: 0
Connection: close


```

## Short description

Complete short HTTP response prepared.

## Trigger condition

A WebRTC deletion, policy XML, redirect, or legacy probe response has been assembled.

## Explanation

The response buffer is logged before sending; it may include a short body after the headers.

## Possible causes

- A WebRTC deletion, policy XML, redirect, or legacy probe response has been assembled.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
