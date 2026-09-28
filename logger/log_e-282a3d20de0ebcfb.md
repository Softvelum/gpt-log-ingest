# Encoded video packet counter reported

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_e
Log level: error
Documentation ID: log_e-282a3d20de0ebcfb

## Log pattern

```text
debugWriteVideoPacket for [%s/%s]
```

## Parameters

- Argument 1 — Application: Output application name, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
debugWriteVideoPacket for [live/encoded]
```

## Short description

Encoded video packet counter reported.

## Trigger condition

Encoder debug logging is enabled and synchronized video-packet write attempts since the last report exceed 1500.

## Explanation

The counter resets before codec-specific packet delivery; the message does not establish delivery success.
