# SDI audio decoder is unavailable

Product: Nimble Streamer
Component: SDI device input
Logging function: log_e
Log level: error
Documentation ID: log_e-ccf446c9830fe7ab

## Log pattern

```text
failed to find audio decoder for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to find audio decoder for [live/camera]
```

## Short description

SDI audio decoder is unavailable.

## Trigger condition

FFmpeg cannot find a decoder for the device audio codec.

## Explanation

The current device decoding operation returns failure.

## Possible causes

- The installed FFmpeg codecs do not provide the device audio decoder.

## What to check

1. Check the device audio codec and installed FFmpeg codec support.

## Recommended action

Provide FFmpeg support for the device audio codec.
