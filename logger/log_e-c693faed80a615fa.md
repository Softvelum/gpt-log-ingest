# Audio sample format conversion failed

Product: Nimble Streamer
Component: Audio frame preparation
Logging function: log_e
Log level: error
Documentation ID: log_e-c693faed80a615fa

## Log pattern

```text
failed to convert audio frame from %d to %d
```

## Parameters

- Argument 1 — Format: FFmpeg sample or pixel format identifier, integer; an enumeration rather than a bit depth.
- Argument 2 — Destination format: Required output FFmpeg sample-format identifier, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to convert audio frame from 0 to 1
```

## Short description

Audio sample format conversion failed.

## Trigger condition

Converting the input frame to the encoder-required sample format returns a nonzero result.

## Explanation

The current frame is not encoded.

## Possible causes

- The stated frame allocation, layout copy, or sample conversion operation failed.

## What to check

1. Check the input audio format, channel layout, and accompanying FFmpeg diagnostics.

## Recommended action

Resolve the reported frame-preparation failure.
