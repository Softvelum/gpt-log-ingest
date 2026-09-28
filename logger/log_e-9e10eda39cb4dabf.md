# RTMP pull cannot find its assigned message stream

Product: Nimble Streamer
Component: RTMP connection transport
Logging function: log_e
Log level: error
Documentation ID: log_e-9e10eda39cb4dabf

## Log pattern

```text
cannot find stream_id %d for client %d
```

## Parameters

- Argument 1 — Stream ID: Assigned RTMP message stream identifier retained for the pull connection.
- Argument 2 — Socket: Operating-system socket descriptor for this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot find stream_id 1 for client 42
```

## Short description

RTMP pull cannot find its assigned message stream.

## Trigger condition

Preparing play or accepting a successful play response cannot find the message stream identified by the earlier createStream result.

## Explanation

The pull connection is shut down instead of attaching the input to an absent message stream.

## Possible causes

- Preparing play or accepting a successful play response cannot find the message stream identified by the earlier createStream result.

## What to check

1. Compare the assigned stream ID with the createStream result and subsequent connection sequence.

## Recommended action

Correct inconsistent stream setup at the remote endpoint or report the missing local stream state.
