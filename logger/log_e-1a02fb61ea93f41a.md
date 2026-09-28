# RTMP control message uses a nonzero message stream ID

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-1a02fb61ea93f41a

## Log pattern

```text
Encoder violates the standard and sends msg_id=%d for RTMP_CHUNK_SIZE_TYPE.add 'rtmp_check_stream_id_for_protocol_control_messages = false' to nimble.conf to FIX issue
```

## Parameters

- Argument 1 — Message stream ID: Numeric RTMP message stream identifier carried by the received message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Encoder violates the standard and sends msg_id=1 for RTMP_CHUNK_SIZE_TYPE.add 'rtmp_check_stream_id_for_protocol_control_messages = false' to nimble.conf to FIX issue
```

## Short description

RTMP control message uses a nonzero message stream ID.

## Trigger condition

A chunk-size control message arrives on control chunk stream 2 with a nonzero message stream ID while message-stream-ID checking is enabled.

## Explanation

The control message is ignored and processing continues. This can leave the peer and server with different control settings.

## Possible causes

- A chunk-size control message arrives on control chunk stream 2 with a nonzero message stream ID while message-stream-ID checking is enabled.

## What to check

1. Check the encoder control-message headers and whether message-stream-ID validation is enabled.

## Recommended action

Configure the encoder to send protocol control messages with message stream ID 0. The compatibility setting named in the message disables this check.
