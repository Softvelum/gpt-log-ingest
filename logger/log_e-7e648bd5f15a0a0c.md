# RTSP listener could not bind its configured endpoint

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-7e648bd5f15a0a0c

## Log pattern

```text
failed to bind rtsp socket %s:%d
```

## Parameters

- Argument 1 — Bind address: Configured local listening IP address.
- Argument 2 — Port: Decimal local RTSP listening port.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to bind rtsp socket 192.0.2.10:554
```

## Short description

RTSP listener could not bind its configured endpoint.

## Trigger condition

Creating a listening socket for a newly configured RTSP interface fails.

## Explanation

That endpoint is not added. The interface configuration is not marked fully synchronized, allowing a later synchronization attempt to retry.

## Possible causes

- Creating a listening socket for a newly configured RTSP interface fails.

## What to check

1. Check whether the configured local address and port can be used for listening.
