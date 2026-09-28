# SDI video decoder is unavailable

Product: Nimble Streamer
Component: SDI device input
Logging function: log_e
Log level: error
Documentation ID: log_e-4d3b3f27a4af8b44

## Log pattern

```text
failed to find video decoder for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to find video decoder for [live/camera]
```

## Short description

SDI video decoder is unavailable.

## Trigger condition

FFmpeg cannot find a decoder for the device video codec.

## Explanation

The current device decoding operation returns failure.

## Possible causes

- The installed FFmpeg codecs do not provide the device video decoder.

## What to check

1. Check the device video codec and installed FFmpeg codec support.

## Recommended action

Provide FFmpeg support for the device video codec.
