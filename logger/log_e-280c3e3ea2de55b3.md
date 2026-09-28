# RTMP publish or FCPublish stream name cannot be decoded

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-280c3e3ea2de55b3

## Log pattern

```text
failed to process stream name for publish command
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process stream name for publish command
```

## Short description

RTMP publish or FCPublish stream name cannot be decoded.

## Trigger condition

The publish or FCPublish request cannot supply its stream name as an AMF string.

## Explanation

The command is rejected and connection processing fails at this field. Truncated data or an unexpected AMF type can cause the read to fail. The same text is used for both publish and FCPublish.

## Possible causes

- The publish or FCPublish request cannot supply its stream name as an AMF string.

## What to check

1. Inspect the stream name field and AMF framing in the incoming publish or FCPublish command.

## Recommended action

Correct the peer command encoding and ensure the full command reaches the server.
