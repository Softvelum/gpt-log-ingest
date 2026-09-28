# RTMP deleteStream stream ID to delete cannot be decoded

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-48cf76903ce3edc2

## Log pattern

```text
failed to process stream id for deleteStream command
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process stream id for deleteStream command
```

## Short description

RTMP deleteStream stream ID to delete cannot be decoded.

## Trigger condition

The deleteStream request cannot supply its stream ID to delete as an AMF number.

## Explanation

The command is rejected and connection processing fails at this field. Truncated data or an unexpected AMF type can cause the read to fail.

## Possible causes

- The deleteStream request cannot supply its stream ID to delete as an AMF number.

## What to check

1. Inspect the stream ID to delete field and AMF framing in the incoming deleteStream command.

## Recommended action

Correct the peer command encoding and ensure the full command reaches the server.
