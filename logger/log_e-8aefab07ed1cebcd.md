# RTMP requested message stream is already allocated

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-8aefab07ed1cebcd

## Log pattern

```text
failed to init message stream, msg_id=%d
```

## Parameters

- Argument 1 — Requested stream ID: Requested AMF numeric stream ID converted to an integer for this log; lookup uses a 16-bit stream ID.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to init message stream, msg_id=1
```

## Short description

RTMP requested message stream is already allocated.

## Trigger condition

An initStream command requests a nonzero stream ID that is already present on this connection after conversion to the supported stream-ID width.

## Explanation

The requested stream cannot be initialized again and connection processing fails.

## Possible causes

- An initStream command requests a nonzero stream ID that is already present on this connection after conversion to the supported stream-ID width.

## What to check

1. Compare the requested ID with streams already allocated on the connection.

## Recommended action

Use an unused stream ID or close the previous stream before reusing its ID.
