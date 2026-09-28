# Client response header write progress

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-2c0dbcec0819618f

## Log pattern

```text
%u of %u B(h) to cli s=%d
```

## Parameters

- Argument 1 — Sent header-buffer bytes: Unsigned cumulative bytes sent from the header buffer.
- Argument 2 — Header-buffer bytes: Native-size unsigned total header-buffer length, incorrectly printed with %u.
- Argument 3 — Client socket: Integer identifying the local client socket.

The total header-string length is a native-size unsigned value supplied to %u; that field is unreliable where size_t is wider than unsigned int.

## Log examples

The total header-string length is a native-size unsigned value supplied to %u; that field is unreliable where size_t is wider than unsigned int.

## Short description

Client response header write progress.

## Trigger condition

A positive write sends more bytes of a fully prepared response header buffer.

## Explanation

The sent count advances and session statistics are updated; the buffer may also contain a short response body.

## Possible causes

- A positive write sends more bytes of a fully prepared response header buffer.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
