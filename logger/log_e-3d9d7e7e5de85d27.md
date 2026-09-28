# RTMP connect command uses a nonzero message stream ID

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-3d9d7e7e5de85d27

## Log pattern

```text
connect command must be sent via message stream id=0
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
connect command must be sent via message stream id=0
```

## Short description

RTMP connect command uses a nonzero message stream ID.

## Trigger condition

The peer sends connect on a message stream other than 0.

## Explanation

The command is rejected and connection processing fails. These connection-level commands are handled only on message stream 0.

## Possible causes

- The peer sends connect on a message stream other than 0.

## What to check

1. Inspect the message stream ID in the connect request.

## Recommended action

Correct the peer to use message stream ID 0 for this command.
