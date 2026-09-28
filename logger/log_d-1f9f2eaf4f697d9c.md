# Streaming response header write progress

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-1f9f2eaf4f697d9c

## Log pattern

```text
%lu of %lu B(h) to cli s=%d
```

## Parameters

- Argument 1 — Sent header bytes: Unsigned int cumulative header bytes, incorrectly printed with %lu.
- Argument 2 — Header bytes: Native-size unsigned total header-buffer length, printed as unsigned long.
- Argument 3 — Client socket: Integer identifying the local client socket.

The sent counter is unsigned int but is passed to %lu, which expects unsigned long; its output is unreliable where these widths differ.

## Log examples

The sent counter is unsigned int but is passed to %lu, which expects unsigned long; its output is unreliable where these widths differ.

## Short description

Streaming response header write progress.

## Trigger condition

A positive write advances the response header while the body is still arriving.

## Explanation

Header progress and access statistics are updated before later writes can send body data.

## Possible causes

- A positive write advances the response header while the body is still arriving.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
