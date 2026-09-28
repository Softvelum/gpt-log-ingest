# SDI audio channel layout copy failed

Product: Nimble Streamer
Component: SDI device input
Logging function: log_e
Log level: error
Documentation ID: log_e-ba68eedf61b422ea

## Log pattern

```text
failed to copy channel layout for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to copy channel layout for [live/camera]
```

## Short description

SDI audio channel layout copy failed.

## Trigger condition

Copying the first decoded audio frame channel layout into the retained input configuration returns a negative result.

## Explanation

The audio frame is rejected before sample-format conversion.

## Possible causes

- The audio channel layout cannot be copied.

## What to check

1. Check the device channel layout and memory availability.

## Recommended action

Correct an invalid layout or restore memory and retry the input.
