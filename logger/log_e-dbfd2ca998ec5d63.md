# AV1 encoder configuration construction failed

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-dbfd2ca998ec5d63

## Log pattern

```text
failed to build_av1_config_record for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to build_av1_config_record for [live/camera]
```

## Short description

AV1 encoder configuration construction failed.

## Trigger condition

Parsing the encoder AV1 sequence header or writing the four-byte AV1 configuration prefix fails.

## Explanation

Codec configuration setup returns failure before passing the record to the output.

## Possible causes

- The AV1 sequence header is invalid or unsupported, or the configuration prefix cannot be written.

## What to check

1. Check the AV1 encoder header and installed encoder compatibility.

## Recommended action

Use valid supported AV1 sequence data.
