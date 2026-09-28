# Video encoder codec configuration is empty

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-88611e3cb8e6ffd2

## Log pattern

```text
failed to get encoder extradata for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get encoder extradata for [live/camera]
```

## Short description

Video encoder codec configuration is empty.

## Trigger condition

While preparing the first encoded output packet, no codec configuration is available from the encoder context or packet side data, and any H.264/HEVC packet fallback is also empty.

## Explanation

Codec configuration publication fails and the encoded packet is not passed to the output writer. The initialization call checks for nonempty configuration before invoking this operation, so it does not normally emit this particular message.

## Possible causes

- No usable codec configuration is available from the encoder context, packet side data or H.264/HEVC packet fallback.

## What to check

1. Check whether the encoder provides configuration for its codec before the first output packet.

## Recommended action

Use an encoder/configuration that supplies valid codec headers.
