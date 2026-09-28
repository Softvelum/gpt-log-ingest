# FFmpeg video flush packet retrieval failed

Product: Nimble Streamer
Component: FFmpeg video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ca35f921d6397fbc

## Log pattern

```text
failed to recv encoded video packet during flush from encoder for [%s]: %s
```

## Parameters

- Argument 1 — Stream: Output stream name, text.
- Argument 2 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to recv encoded video packet during flush from encoder for [live/camera]: operation failed
```

## Short description

FFmpeg video flush packet retrieval failed.

## Trigger condition

During final flushing, packet retrieval returns a nonzero result other than end of stream.

## Explanation

Flush returns failure before all remaining encoded packets are delivered.

## Possible causes

- The encoder reports the included error while accepting input or returning output.

## What to check

1. Check the FFmpeg error text and preceding encoder/input diagnostics.

## Recommended action

Correct the reported encoding problem and restart the session if its state cannot recover.
