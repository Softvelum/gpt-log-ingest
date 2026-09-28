# FFmpeg audio decoder receive failed

Product: Nimble Streamer
Component: FFmpeg audio and video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-de46af760ac1ca58

## Log pattern

```text
failed to recv decoded audio from decoder [%s]: %s
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to recv decoded audio from decoder [live/camera]: operation failed
```

## Short description

FFmpeg audio decoder receive failed.

## Trigger condition

Retrieving a decoded frame returns an error after ordinary need-more-input handling.

## Explanation

This decode operation returns success after logging, allowing later input to continue; the failed output is not forwarded.

## Possible causes

- FFmpeg reported the included packet-decoding error.

## What to check

1. Check the input audio packet and the FFmpeg error text.

## Recommended action

Correct malformed or incompatible input according to the reported decoder error.
