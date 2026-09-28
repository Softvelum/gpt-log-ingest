# RTSP outgoing message could not be buffered

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-eefb5cc04dcedf22

## Log pattern

```text
failed to write buffer to client: %d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to write buffer to client: 25
```

## Short description

RTSP outgoing message could not be buffered.

## Trigger condition

Appending a complete outgoing message to the connection buffer fails.

## Explanation

The connection begins shutdown without queuing the requested message. The outgoing buffer has a configured maximum of four MiB in this path.

## Possible causes

- Appending a complete outgoing message to the connection buffer fails.

## What to check

1. Check whether the pending outgoing data exceeds the buffer capacity.

## Related log messages

- [RTSP output fragment could not be buffered](log_e-f66e95444325c1b8.md)
