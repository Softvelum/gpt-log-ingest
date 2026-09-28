# Origin socket removed from processing

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-77ca82f6f4fcbb0c

## Log pattern

```text
removed origin s=%d from processing (active=%d)
```

## Parameters

- Argument 1 — Origin socket: Integer identifying the local origin socket.
- Argument 2 — Remaining map entries: Native-size unsigned number of registered socket/session entries after removal, incorrectly printed with %d.

The active-map size is a native-size unsigned count supplied to %d, so that field is unreliable on platforms where size_t is wider than int.

## Log examples

The active-map size is a native-size unsigned count supplied to %d, so that field is unreliable on platforms where size_t is wider than int.

## Short description

Origin socket removed from processing.

## Trigger condition

The origin socket is removed from the event selector and active-session map during cleanup.

## Explanation

The remaining map size is logged before closing the origin socket.

## Possible causes

- The origin socket is removed from the event selector and active-session map during cleanup.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
