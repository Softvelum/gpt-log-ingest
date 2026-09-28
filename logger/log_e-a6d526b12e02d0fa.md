# RTMP deleteStream refers to an unknown stream

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-a6d526b12e02d0fa

## Log pattern

```text
unknown message stream id for deleteStream, msg_id=%d
```

## Parameters

- Argument 1 — Requested stream ID: Intended AMF numeric stream ID to delete; the floating-point argument does not match the integer conversion, so the printed value is unreliable.

## Log examples

No reliable rendered example is available because the integer conversion receives a floating-point argument.

## Short description

RTMP deleteStream refers to an unknown stream.

## Trigger condition

The requested deleteStream ID does not resolve to an existing message stream on the connection.

## Explanation

The delete command is rejected and connection processing fails. The numeric value printed by this message is unreliable because its floating-point argument is formatted as an integer.

## Possible causes

- The requested deleteStream ID does not resolve to an existing message stream on the connection.

## What to check

1. Check the stream ID in the actual deleteStream request and whether the connection previously allocated it.

## Recommended action

Delete only a message stream that is still allocated on the connection.
