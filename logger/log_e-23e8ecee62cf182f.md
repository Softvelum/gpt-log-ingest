# FFmpeg encoded NAL conversion failed

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-23e8ecee62cf182f

## Log pattern

```text
failed to convert annexb buffer to avcc for [%s]
```

## Parameters

- Argument 1 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to convert annexb buffer to avcc for [live/camera]
```

## Short description

FFmpeg encoded NAL conversion failed.

## Trigger condition

An H.264 or HEVC output packet is not already length-prefixed, and converting its Annex B payload fails.

## Explanation

The packet is not delivered to the output writer; the current frame write or flush returns failure.

## Possible causes

- The packet has fewer than three bytes or the conversion buffer cannot grow to contain its NAL units.

## What to check

1. Check encoded packet validity, packet size and host-buffer resources.

## Recommended action

Correct invalid encoder output or restore buffer capacity.
