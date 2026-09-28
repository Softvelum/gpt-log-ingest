# Origin header bytes received

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-b8c1ede37f6ce646

## Log pattern

```text
received %d (total %d) header bytes from origin s=%d
```

## Parameters

- Argument 1 — Read bytes: Native-size signed byte count incorrectly printed as int.
- Argument 2 — Displayed total: Buffered byte count plus this read; the cache downloader has already included this read once.
- Argument 3 — Origin socket: Integer identifying the local origin socket.

The read count and the displayed sum use a native-size signed type with %d conversions. They are unreliable on 64-bit builds. The cache downloader also adds the read length twice in the displayed total.

## Log examples

The read count and the displayed sum use a native-size signed type with %d conversions. They are unreliable on 64-bit builds. The cache downloader also adds the read length twice in the displayed total.

## Short description

Origin header bytes received.

## Trigger condition

A positive read adds data while receiving an origin header or buffered playlist.

## Explanation

The HTTP or advertising session continues parsing; the cache-downloader total is overstated because its running count already includes this read.

## Possible causes

- A positive read adds data while receiving an origin header or buffered playlist.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
