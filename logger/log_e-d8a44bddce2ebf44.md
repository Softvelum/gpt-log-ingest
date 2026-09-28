# FFmpeg video decoder send failed

Product: Nimble Streamer
Component: FFmpeg audio and video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-d8a44bddce2ebf44

## Log pattern

```text
failed to send video pkt to decoder [%s]: %s 
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send video pkt to decoder [live/camera]: operation failed 
```

## Short description

FFmpeg video decoder send failed.

## Trigger condition

Submitting a packet returns a nonzero result other than the request to drain decoder output first.

## Explanation

This decode operation returns success after logging, allowing later input to continue; the failed output is not forwarded.

## Possible causes

- FFmpeg reported the included packet-decoding error.

## What to check

1. Check the input video packet and the FFmpeg error text.

## Recommended action

Correct malformed or incompatible input according to the reported decoder error.
