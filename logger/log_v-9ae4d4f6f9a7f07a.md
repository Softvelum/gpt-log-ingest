# Cached or forwarded response headers ready

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-9ae4d4f6f9a7f07a

## Log pattern

```text
client s=%d response:
%.*s<...>
```

## Parameters

- Argument 1 — Client socket: Integer identifying the local client socket.
- Argument 2 — Header byte limit: Native-size unsigned header length incorrectly used as an int precision.
- Argument 3 — Response headers: Text of the assembled HTTP response headers.

The %.*s precision is supplied as the native-size unsigned header length instead of int. It is not a portable exact rendering on 64-bit builds.

## Log examples

The %.*s precision is supplied as the native-size unsigned header length instead of int. It is not a portable exact rendering on 64-bit builds.

## Short description

Cached or forwarded response headers ready.

## Trigger condition

A cached or fragmented media response has headers ready, or an origin response copy has no buffered body.

## Explanation

The headers are logged with a literal body-omission marker before client output.

## Possible causes

- A cached or fragmented media response has headers ready, or an origin response copy has no buffered body.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
