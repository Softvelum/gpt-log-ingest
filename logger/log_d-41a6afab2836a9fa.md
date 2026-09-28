# Origin request transmission progress

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-41a6afab2836a9fa

## Log pattern

```text
%u of %u bytes written to origin s=%d
```

## Parameters

- Argument 1 — Sent bytes: Unsigned cumulative request bytes written; stream checks explicitly narrow this count to 32 bits.
- Argument 2 — Request bytes: Unsigned total request size in bytes, explicitly narrowed to 32 bits.
- Argument 3 — Origin socket: Integer identifying the local origin socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
78 of 78 bytes written to origin s=17
```

## Short description

Origin request transmission progress.

## Trigger condition

A positive socket write advances an HTTP request, advertising request, cache request, or stream-check request.

## Explanation

The running sent-byte count is logged; completion switches to response input where appropriate.

## Possible causes

- A positive socket write advances an HTTP request, advertising request, cache request, or stream-check request.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
