# FFmpeg encoded video retrieval failed

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ee7ef1f6c0728b7f

## Log pattern

```text
failed to recv encoded video packet from encoder for [%s]: %s
```

## Parameters

- Argument 1 — Stream: Output stream name, text.
- Argument 2 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to recv encoded video packet from encoder for [live/camera]: operation failed
```

## Short description

FFmpeg encoded video retrieval failed.

## Trigger condition

Receiving an encoded packet returns a nonzero result other than the normal request for more input.

## Explanation

The current frame write fails before delivering another packet.

## Possible causes

- The encoder reports the included error while accepting input or returning output.

## What to check

1. Check the FFmpeg error text and preceding encoder/input diagnostics.

## Recommended action

Correct the reported encoding problem and restart the session if its state cannot recover.
