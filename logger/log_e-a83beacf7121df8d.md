# RTMP live audio configuration cannot be sent

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-a83beacf7121df8d

## Log pattern

```text
failed to send audio sequence header to client: %d
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send audio sequence header to client: 42
```

## Short description

RTMP live audio configuration cannot be sent.

## Trigger condition

The output operation fails while sending the current AAC, Opus, or FLAC sequence header before live audio.

## Explanation

The playback send operation returns failure and its caller shuts down the connection. Ordinary partial sends or readiness waits use a continuation path.

## Possible causes

- The output operation fails while sending the current AAC, Opus, or FLAC sequence header before live audio.

## What to check

1. Check preceding socket or TLS errors and whether the playback peer is still connected.

## Recommended action

Resolve the transport failure before restarting playback.
