# Origin connection established

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c0c5db23ecb8b653

## Log pattern

```text
connected to origin s=%d
```

## Parameters

- Argument 1 — Origin socket: Integer identifying the local origin socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connected to origin s=17
```

## Short description

Origin connection established.

## Trigger condition

A writable event for an origin connection is followed by a successful socket-status check with zero error.

## Explanation

The HTTP request can now be sent; client, cache-download, and advertising sessions use this message.

## Possible causes

- A writable event for an origin connection is followed by a successful socket-status check with zero error.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
