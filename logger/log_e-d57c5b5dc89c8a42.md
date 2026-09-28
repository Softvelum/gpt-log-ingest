# RTMP play command-object placeholder cannot be decoded

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-d57c5b5dc89c8a42

## Log pattern

```text
failed to process null for play command
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process null for play command
```

## Short description

RTMP play command-object placeholder cannot be decoded.

## Trigger condition

The play request cannot supply its command-object placeholder as an AMF null value.

## Explanation

The command is rejected and connection processing fails at this field. Truncated data or an unexpected AMF type can cause the read to fail.

## Possible causes

- The play request cannot supply its command-object placeholder as an AMF null value.

## What to check

1. Inspect the command-object placeholder field and AMF framing in the incoming play command.

## Recommended action

Correct the peer command encoding and ensure the full command reaches the server.
