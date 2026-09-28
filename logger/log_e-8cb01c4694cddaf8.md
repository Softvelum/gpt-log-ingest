# RTMP TLS response cannot be sent

Product: Nimble Streamer
Component: RTMP connection transport
Logging function: log_e
Log level: error
Documentation ID: log_e-8cb01c4694cddaf8

## Log pattern

```text
failed to ssl_send buffer to client: %d
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to ssl_send buffer to client: 42
```

## Short description

RTMP TLS response cannot be sent.

## Trigger condition

Sending pending RTMP response bytes over TLS reports failure, either during connection setup or during playback replies.

## Explanation

The current response cannot complete. Setup starts shutdown immediately; playback propagates failure to its sender.

## Possible causes

- Sending pending RTMP response bytes over TLS reports failure, either during connection setup or during playback replies.

## What to check

1. Check associated TLS or socket errors and whether the peer closed the connection.

## Recommended action

Resolve the transport failure before establishing another session.
