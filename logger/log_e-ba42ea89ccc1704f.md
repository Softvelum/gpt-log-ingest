# FLAC message has no audio payload

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ba42ea89ccc1704f

## Log pattern

```text
wrong FLAC packet size %d, [%s/%s]
```

## Parameters

- Argument 1 — Payload size: Total message size minus RTMP header size, unsigned byte count printed as signed decimal; an undersized header can wrap the subtraction.
- Argument 2 — Application: Application name, text.
- Argument 3 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong FLAC packet size 0, [live/camera]
```

## Short description

FLAC message has no audio payload.

## Trigger condition

The total FLAC message size is less than or equal to its RTMP header size.

## Explanation

The message is rejected before it is passed to the audio decoder.

## Possible causes

- The input message is truncated or contains only its packet header.

## What to check

1. Check the upstream FLAC framing and header size.

## Recommended action

Provide complete FLAC packets with payload after the header.
