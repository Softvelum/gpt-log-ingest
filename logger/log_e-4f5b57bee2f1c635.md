# SDI video frame submission failed

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-4f5b57bee2f1c635

## Log pattern

```text
failed to send video frame to converter for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send video frame to converter for [live/camera]
```

## Short description

SDI video frame submission failed.

## Trigger condition

Sending a video frame to the output encoder returns a negative result.

## Explanation

The current output frame operation returns failure.

## Possible causes

- The output encoder cannot accept the video frame.

## What to check

1. Check the source video frame format and preceding FFmpeg diagnostics.

## Recommended action

Correct the source media or restart the output.
