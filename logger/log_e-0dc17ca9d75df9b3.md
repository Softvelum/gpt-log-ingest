# SDI audio decoder parameters were rejected

Product: Nimble Streamer
Component: SDI device input
Logging function: log_e
Log level: error
Documentation ID: log_e-0dc17ca9d75df9b3

## Log pattern

```text
failed to set audio decoder params for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set audio decoder params for [live/camera]
```

## Short description

SDI audio decoder parameters were rejected.

## Trigger condition

Copying device audio stream parameters into the decoder returns a negative result.

## Explanation

The current device decoding operation returns failure.

## Possible causes

- The device audio parameters cannot be copied into the decoder.

## What to check

1. Check the device audio format and accompanying FFmpeg diagnostics.

## Recommended action

Correct the device format and retry the input.
