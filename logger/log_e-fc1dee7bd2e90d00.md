# MPEG-TS origin request send failed

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-fc1dee7bd2e90d00

## Log pattern

```text
failed to send request to mpeg2ts origin s=%d url='%s'
```

## Parameters

- Argument 1 — socket: Origin connection socket descriptor.
- Argument 2 — url: Current origin URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send request to mpeg2ts origin s=42 url='https://origin.example/live.ts'
```

## Short description

MPEG-TS origin request send failed.

## Trigger condition

Sending the prepared MPEG-TS HTTP request reports failure.

## Explanation

The input requests shutdown and returns without entering response reception.

## Possible causes

- The socket send helper returned failure.

## What to check

1. Check the origin connection and preceding socket/TLS diagnostics.

## Recommended action

Resolve the origin connection failure.
