# FFmpeg audio decoder resend failed

Product: Nimble Streamer
Component: FFmpeg audio and video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-b50acd66a3e56c45

## Log pattern

```text
failed to send again audio pkt to decoder [%s]: %s 
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send again audio pkt to decoder [live/camera]: operation failed 
```

## Short description

FFmpeg audio decoder resend failed.

## Trigger condition

The initial submission required draining output, and the subsequent packet resubmission still returns a nonzero result.

## Explanation

A repeated need-to-drain result reports failure; any other error returns success after logging so later input can continue.

## Possible causes

- FFmpeg reported the included packet-decoding error.

## What to check

1. Check the input audio packet and the FFmpeg error text.

## Recommended action

Correct malformed or incompatible input according to the reported decoder error.
