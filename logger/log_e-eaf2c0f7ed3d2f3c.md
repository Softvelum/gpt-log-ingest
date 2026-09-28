# SDI video packet retrieval failed

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-eaf2c0f7ed3d2f3c

## Log pattern

```text
failed to receive video frame from converter for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to receive video frame from converter for [live/camera]
```

## Short description

SDI video packet retrieval failed.

## Trigger condition

Receiving an encoded video packet fails with an error other than end of stream or the request for more input.

## Explanation

The current output frame operation returns failure.

## Possible causes

- The output encoder fails while producing a video packet.

## What to check

1. Check preceding FFmpeg diagnostics.

## Recommended action

Correct the reported encoding problem and retry.
