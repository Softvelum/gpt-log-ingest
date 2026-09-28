# H.264 output configuration is missing

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_e
Log level: error
Documentation ID: log_e-08ccfae3c4bff955

## Log pattern

```text
no avc config record for [%s/%s]
```

## Parameters

- Argument 1 — Application: Output application name, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
no avc config record for [live/encoded]
```

## Short description

H.264 output configuration is missing.

## Trigger condition

The first encoded H.264 packet needs output stream setup but no codec configuration record has been supplied.

## Explanation

The packet is not delivered and video output initialization fails.

## Possible causes

- The selected encoder has not produced or forwarded its required codec configuration before the packet.

## What to check

1. Check preceding encoder header-generation messages and the chosen codec.

## Recommended action

Use an encoder setup that provides complete codec configuration before output packets.
