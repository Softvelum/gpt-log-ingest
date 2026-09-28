# SDI audio decoder opening failed

Product: Nimble Streamer
Component: SDI device input
Logging function: log_e
Log level: error
Documentation ID: log_e-448c5a9aca4a6d25

## Log pattern

```text
failed to open audio decoder params for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open audio decoder params for [live/camera]
```

## Short description

SDI audio decoder opening failed.

## Trigger condition

Opening the device audio decoder returns a nonzero result.

## Explanation

The current device decoding operation returns failure.

## Possible causes

- The decoder rejects the supplied device audio configuration.

## What to check

1. Check the device audio format and accompanying FFmpeg diagnostics.

## Recommended action

Use a supported device format and retry the input.
