# H.264 encoder configuration lengths are invalid

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-2bd566ea78dcc719

## Log pattern

```text
failed to parse avcc sps/pps for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse avcc sps/pps for [live/camera]
```

## Short description

H.264 encoder configuration lengths are invalid.

## Trigger condition

Parsing length-prefixed H.264 configuration NAL units finds a declared length beyond the remaining data or an empty NAL unit.

## Explanation

The output codec configuration is rejected and initialization or packet delivery returns failure.

## Possible causes

- The encoder configuration has malformed NAL lengths or empty payloads.

## What to check

1. Check the encoder output configuration and preceding codec diagnostics.

## Recommended action

Provide valid H.264 codec configuration.
