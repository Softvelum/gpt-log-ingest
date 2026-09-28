# RTMP publish or FCPublish command-object placeholder cannot be decoded

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-31bace5211cadb83

## Log pattern

```text
failed to process null for publish command
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process null for publish command
```

## Short description

RTMP publish or FCPublish command-object placeholder cannot be decoded.

## Trigger condition

The publish or FCPublish request cannot supply its command-object placeholder as an AMF null value.

## Explanation

The command is rejected and connection processing fails at this field. Truncated data or an unexpected AMF type can cause the read to fail. The same text is used for both publish and FCPublish.

## Possible causes

- The publish or FCPublish request cannot supply its command-object placeholder as an AMF null value.

## What to check

1. Inspect the command-object placeholder field and AMF framing in the incoming publish or FCPublish command.

## Recommended action

Correct the peer command encoding and ensure the full command reaches the server.
