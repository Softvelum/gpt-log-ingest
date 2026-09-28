# Response file closed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-d85cf46a7e54a44f

## Log pattern

```text
closed file fd=%d
```

## Parameters

- Argument 1 — File descriptor: Integer descriptor of the response file that was closed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
closed file fd=23
```

## Short description

Response file closed.

## Trigger condition

An open response file is successfully closed during request cleanup or reply destruction.

## Explanation

The file is no longer used to send response data. HTTP/1 cleanup also resets file offsets and releases output buffers.

## Possible causes

- An open response file is successfully closed during request cleanup or reply destruction.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
