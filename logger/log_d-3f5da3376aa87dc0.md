# Origin circular-buffer read progress

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-3f5da3376aa87dc0

## Log pattern

```text
received %d (total %d of %u) body bytes from origin s=%d
```

## Parameters

- Argument 1 — Read bytes: Native-size signed byte count incorrectly printed as int.
- Argument 2 — Received bytes: Unsigned 64-bit prior received bytes plus this read, incorrectly printed as int.
- Argument 3 — Expected bytes: Unsigned 64-bit expected body size incorrectly printed as unsigned int.
- Argument 4 — Origin socket: Integer identifying the local origin socket.

The first count is a native-size signed read result and the next two counts are unsigned 64-bit values, but the format uses %d, %d, and %u. The displayed counts are unreliable on 64-bit builds.

## Log examples

The first count is a native-size signed read result and the next two counts are unsigned 64-bit values, but the format uses %d, %d, and %u. The displayed counts are unreliable on 64-bit builds.

## Short description

Origin circular-buffer read progress.

## Trigger condition

The pass-through response buffer successfully reads more origin-body data.

## Explanation

The session logs this read and intended cumulative progress, then parses and accounts for the body bytes.

## Possible causes

- The pass-through response buffer successfully reads more origin-body data.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
