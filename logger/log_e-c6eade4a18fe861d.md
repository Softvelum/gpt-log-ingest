# FFmpeg video decoder receive failed

Product: Nimble Streamer
Component: FFmpeg audio and video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-c6eade4a18fe861d

## Log pattern

```text
failed to recv decoded video frame [%s]: %s 
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to recv decoded video frame [live/camera]: operation failed 
```

## Short description

FFmpeg video decoder receive failed.

## Trigger condition

Retrieving a decoded frame returns an error after ordinary need-more-input handling. End of decoded video is handled separately without this message.

## Explanation

This decode operation returns success after logging, allowing later input to continue; the failed output is not forwarded.

## Possible causes

- FFmpeg reported the included packet-decoding error.

## What to check

1. Check the input video packet and the FFmpeg error text.

## Recommended action

Correct malformed or incompatible input according to the reported decoder error.
