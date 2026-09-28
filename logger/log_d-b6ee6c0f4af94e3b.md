# RTSP publishing worker started

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_d
Log level: debug
Documentation ID: log_d-b6ee6c0f4af94e3b

## Log pattern

```text
RtspPublisherThread started
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
RtspPublisherThread started
```

## Short description

RTSP publishing worker started.

## Trigger condition

The RTSP publishing worker enters its main processing routine.

## Explanation

The worker proceeds to synchronize configured connections or listeners and process socket events. Startup does not prove that any endpoint is connected.

## Possible causes

- Normal worker startup.

## Related log messages

- [RTSP publishing worker stopped](log_d-68c5344fa1b09bf0.md)
