# RTP shutdown requested

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_d
Log level: debug
Documentation ID: log_d-cddde1284dcc9a02

## Log pattern

```text
rtp shutdown, s=%d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rtp shutdown, s=25
```

## Short description

RTP shutdown requested.

## Trigger condition

Shutdown is requested for the RTP connection.

## Explanation

The message occurs before the repeated-shutdown guard, so it can appear more than once. The first effective request also shuts down the parent RTSP input and the connection.

## Possible causes

- Shutdown is requested for the RTP connection.

## What to check

1. Check earlier messages for the event that requested shutdown.

## Related log messages

- [RTP connection destroyed](log_d-65e98799421d79f2.md)
- [RTP connection created](log_d-9b8216180c95fbd6.md)
