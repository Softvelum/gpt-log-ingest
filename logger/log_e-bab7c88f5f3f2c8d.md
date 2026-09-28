# Sent origin request count exceeds its length

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-bab7c88f5f3f2c8d

## Log pattern

```text
ClientSession::OnSockOut origin s=%d assertion failed (%d < %d)
```

## Parameters

- Argument 1 — Origin socket: Integer identifying the local origin socket.
- Argument 2 — Request bytes: Request string length in bytes cast to int.
- Argument 3 — Sent bytes: Cumulative bytes sent cast to int.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ClientSession::OnSockOut origin s=17 assertion failed (78 < 80)
```

## Short description

Sent origin request count exceeds its length.

## Trigger condition

After a successful write, the cumulative sent count exceeds the request string length in an HTTP client or advertising downloader.

## Explanation

The session is marked finished because its transmission counters are inconsistent.

## Possible causes

- After a successful write, the cumulative sent count exceeds the request string length in an HTTP client or advertising downloader.

## What to check

1. Correlate earlier write counts for the same socket; this is an internal request-accounting failure.

## Recommended action

Correlate earlier write counts for the same socket; this is an internal request-accounting failure.
