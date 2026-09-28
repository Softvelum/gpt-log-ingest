# Origin connection in progress

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-6fdfe5e8e3eafbd7

## Log pattern

```text
connecting to origin='%s:%d' s=%d
```

## Parameters

- Argument 1 — Origin hostname: Text hostname without the port.
- Argument 2 — Origin port: Integer TCP port.
- Argument 3 — Origin socket: Integer identifying the local origin socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connecting to origin='media.example:443' s=17
```

## Short description

Origin connection in progress.

## Trigger condition

A nonblocking connection to an origin returns the expected in-progress status.

## Explanation

The session proceeds to descriptor checks, optional TLS initialization, and writable-event registration.

## Possible causes

- A nonblocking connection to an origin returns the expected in-progress status.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
