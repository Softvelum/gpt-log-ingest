# AVC configuration has no usable sequence header

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-70f0bc262eeb4a72

## Log pattern

```text
failed to build avc config record for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to build avc config record for [live/camera]
```

## Short description

AVC configuration has no usable sequence header.

## Trigger condition

After collecting H.264 NAL units, the sequence parameter set contains fewer than four bytes.

## Explanation

Building the AVC configuration record fails, preventing codec setup or packet delivery.

## Possible causes

- The encoder data has no SPS or its SPS is too short to provide profile and level information.

## What to check

1. Check whether a complete H.264 SPS precedes video output.

## Recommended action

Provide a complete H.264 sequence header.
