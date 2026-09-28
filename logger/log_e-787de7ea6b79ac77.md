# SDI audio packet write failed

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-787de7ea6b79ac77

## Log pattern

```text
failed to write audio packet for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to write audio packet for [live/camera]
```

## Short description

SDI audio packet write failed.

## Trigger condition

Writing an encoded audio packet to the device output returns a negative result.

## Explanation

The current output frame operation returns failure.

## Possible causes

- The device output cannot write the audio packet.

## What to check

1. Check device availability and nearby output diagnostics.

## Recommended action

Restore device output operation and retry.
