# VPX frame encoding failed

Product: Nimble Streamer
Component: VP8 and VP9 encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-9bb8c0a9212f1b2c

## Log pattern

```text
failed to encode frame: %s
```

## Parameters

- Argument 1 — Codec error: libvpx description of the returned error, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to encode frame: Invalid parameter
```

## Short description

VPX frame encoding failed.

## Trigger condition

libvpx returns a non-success result when encoding the submitted video frame.

## Explanation

The operation returns failure before draining any encoded packets from that call.

## Possible causes

- The codec cannot encode the frame with the current configuration or available resources.

## What to check

1. Check the error description and current input frame and encoding settings.

## Recommended action

Correct the indicated codec problem and restart the affected encoder.
