# FFmpeg video frame submission failed

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-fbebc52529e7b66b

## Log pattern

```text
failed to send video frame to encoder for [%s]: %s
```

## Parameters

- Argument 1 — Stream: Output stream name, text.
- Argument 2 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send video frame to encoder for [live/camera]: operation failed
```

## Short description

FFmpeg video frame submission failed.

## Trigger condition

Initial frame submission returns a nonzero result other than the request to drain output.

## Explanation

The current frame write fails before draining encoded packets.

## Possible causes

- The encoder reports the included error while accepting input or returning output.

## What to check

1. Check the FFmpeg error text and preceding encoder/input diagnostics.

## Recommended action

Correct the reported encoding problem and restart the session if its state cannot recover.
