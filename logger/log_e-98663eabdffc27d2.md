# VAAPI encoder input has no hardware frame context

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-98663eabdffc27d2

## Log pattern

```text
vaapi enc requires vaapi frames
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
vaapi enc requires vaapi frames
```

## Short description

VAAPI encoder input has no hardware frame context.

## Trigger condition

The selected FFmpeg encoder name denotes VAAPI, but the input frame carries no hardware-frames context.

## Explanation

Encoder initialization fails before opening the codec.

## Possible causes

- The upstream input has not supplied hardware-backed VAAPI frames.

## What to check

1. Check the decoder/filter hardware-frame path into this encoder.

## Recommended action

Supply VAAPI frames with their hardware context.
