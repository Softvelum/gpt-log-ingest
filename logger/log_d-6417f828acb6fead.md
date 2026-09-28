# RTSP input connection created

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_d
Log level: debug
Documentation ID: log_d-6417f828acb6fead

## Log pattern

```text
rtsp open, s=%d url=%s
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.
- Argument 2 — URL: RTSP stream or track URL for the request.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rtsp open, s=25 url=rtsp://192.0.2.1/live/camera
```

## Short description

RTSP input connection created.

## Trigger condition

An RTSP input connection is initialized with its resolved source URL.

## Explanation

The client next sends an HTTP GET for HTTP tunneling, or RTSP OPTIONS for direct operation. The message precedes successful protocol negotiation.

## Possible causes

- Normal RTSP input startup.

## Related log messages

- [RTSP input connection destroyed](log_d-85c7038dc68d610f.md)
