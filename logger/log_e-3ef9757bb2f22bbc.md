# RTSP TLS socket send failed

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-3ef9757bb2f22bbc

## Log pattern

```text
failed to ssl_send buffer to s=%d errno=%d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.
- Argument 2 — Error number: Integer error value reported by the socket operation; its meaning depends on the operating system.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to ssl_send buffer to s=25 errno=32
```

## Short description

RTSP TLS socket send failed.

## Trigger condition

The TLS send operation returns zero, or a negative result that does not request another read or write event.

## Explanation

Sending returns failure. Normal TLS retry conditions do not produce this message; the event-processing caller requests shutdown.

## Possible causes

- The TLS send operation returns zero, or a negative result that does not request another read or write event.

## What to check

1. Check the socket error and TLS peer connection state.

## Related log messages

- [RTSP plain socket send failed](log_e-2f914c06074d3eb3.md)
