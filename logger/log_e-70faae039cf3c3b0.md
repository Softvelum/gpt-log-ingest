# RTMP metadata contains an unreadable field

Product: Nimble Streamer
Component: RTMP media input
Logging function: log_e
Log level: error
Documentation ID: log_e-70faae039cf3c3b0

## Log pattern

```text
cannot process AMF. cannot skip %s(%s)
```

## Parameters

- Argument 1 — Field name: Name of the metadata field whose value could not be skipped.
- Argument 2 — Stream: Application and stream name associated with this RTMP message stream; it can be empty before association.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot process AMF. cannot skip customField(live/camera)
```

## Short description

RTMP metadata contains an unreadable field.

## Trigger condition

An unhandled metadata field cannot be skipped, including an audiocodecid field with an unexpected AMF type.

## Explanation

Further metadata field decoding stops. The handler still attempts to build audio metadata and apply values decoded before the error.

## Possible causes

- An unhandled metadata field cannot be skipped, including an audiocodecid field with an unexpected AMF type.

## What to check

1. Inspect the named field and its AMF type and length in the encoder metadata.

## Recommended action

Correct the malformed field or use a supported metadata encoding.
