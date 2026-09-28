# RTMP publishing command transaction fails

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-28be42ae0d07ecce

## Log pattern

```text
command[%d] failed for s: %d, %s:%d/%s/%s
```

## Parameters

- Argument 1 — Transaction ID: AMF transaction ID converted to an integer for display.
- Argument 2 — Socket: Operating-system socket descriptor for the affected connection.
- Argument 3 — Destination host: Publishing destination hostname or address.
- Argument 4 — Destination port: Publishing destination TCP port.
- Argument 5 — Application: Destination application name.
- Argument 6 — Stream: Destination stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
command[2] failed for s: 42, destination.example:1935/live/camera
```

## Short description

RTMP publishing command transaction fails.

## Trigger condition

The outbound publisher reports a failed command response with a transaction ID other than 0 or 1.

## Explanation

This message records the integer form of the transaction ID and destination. The surrounding response handler determines termination.

## Possible causes

- The outbound publisher reports a failed command response with a transaction ID other than 0 or 1.

## What to check

1. Match the transaction to the outstanding publishing setup command and inspect the remote response.

## Recommended action

Resolve the reported destination rejection.
