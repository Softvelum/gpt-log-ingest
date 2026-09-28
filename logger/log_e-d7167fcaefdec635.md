# Enhanced RTMP AAC media precedes its sequence header

Product: Nimble Streamer
Component: RTMP media input
Logging function: log_e
Log level: error
Documentation ID: log_e-d7167fcaefdec635

## Log pattern

```text
[AM] aac data MUST follow aac sequence header for (%s)
```

## Parameters

- Argument 1 — Stream: Application and stream name associated with this RTMP message stream; it can be empty before association.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[AM] aac data MUST follow aac sequence header for (live/camera)
```

## Short description

Enhanced RTMP AAC media precedes its sequence header.

## Trigger condition

A AAC coded-audio packet arrives before enhanced audio initialization.

## Explanation

The packet is rejected and connection processing fails before audio delivery.

## Possible causes

- A AAC coded-audio packet arrives before enhanced audio initialization.

## What to check

1. Check whether a AAC sequence-start packet was sent and accepted before coded frames.

## Recommended action

Send the AAC sequence header before coded audio packets.
