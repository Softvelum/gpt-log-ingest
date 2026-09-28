# RTMP media arrives without an active registered stream

Product: Nimble Streamer
Component: RTMP media input
Logging function: log_e
Log level: error
Documentation ID: log_e-9c44e5b3cf3e864b

## Log pattern

```text
stream not yet registered or unregistered(%s)
```

## Parameters

- Argument 1 — Stream: Application and stream name associated with this RTMP message stream; it can be empty before association.

## Log examples

Illustrative examples; these are not captured server logs.

```text
stream not yet registered or unregistered(live/camera)
```

## Short description

RTMP media arrives without an active registered stream.

## Trigger condition

A supported metadata, audio, or video message reaches a stream whose live registration is missing or already inactive.

## Explanation

The media handler rejects the message. Audio and video failures stop normal connection processing. The normal AMF0 data dispatcher ignores the metadata handler failure, while replay of probed metadata treats it as failure.

## Possible causes

- A supported metadata, audio, or video message reaches a stream whose live registration is missing or already inactive.

## What to check

1. Check whether publication succeeded and whether the live stream was removed before these messages arrived.

## Recommended action

Establish an active publishing stream before sending its media and metadata.
