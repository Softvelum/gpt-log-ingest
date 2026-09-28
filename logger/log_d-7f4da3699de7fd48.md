# RTMP or RTSP socket reports a error

Product: Nimble Streamer
Component: RTMP connection transport
Logging function: log_d
Log level: debug
Documentation ID: log_d-7f4da3699de7fd48

## Log pattern

```text
EPOLLERR received from client: %d
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
EPOLLERR received from client: 42
```

## Short description

RTMP or RTSP socket reports a error.

## Trigger condition

The event system reports a socket error; the RTSP error branch applies only to a non-UDP transport.

## Explanation

The connection is immediately marked shut down. RTMP emits this message only when a more specific network-error logger does not handle the event.

## Possible causes

- The event system reports a socket error; the RTSP error branch applies only to a non-UDP transport.

## What to check

1. Check the connection peer and earlier network messages for the reason the transport ended.

## Recommended action

Resolve persistent peer or transport failures before reconnecting.
