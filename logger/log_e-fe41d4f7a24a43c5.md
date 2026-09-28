# SDI video packet submission failed

Product: Nimble Streamer
Component: SDI device input
Logging function: log_e
Log level: error
Documentation ID: log_e-fe41d4f7a24a43c5

## Log pattern

```text
failed to convert video packet for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to convert video packet for [live/camera]
```

## Short description

SDI video packet submission failed.

## Trigger condition

Submitting a device video packet to the decoder fails with an error other than the request to drain output.

## Explanation

The current device decoding operation returns failure.

## Possible causes

- The decoder rejects a device video packet.

## What to check

1. Check the device video input and preceding decoder diagnostics.

## Recommended action

Correct or restart the failing device input.
