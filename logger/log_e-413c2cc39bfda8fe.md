# Audio encoder input submission failed

Product: Nimble Streamer
Component: FFmpeg audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-413c2cc39bfda8fe

## Log pattern

```text
failed to send audio frame to encoder: %s
```

## Parameters

- Argument 1 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send audio frame to encoder: operation failed
```

## Short description

Audio encoder input submission failed.

## Trigger condition

Submitting an audio frame returns a nonzero error other than the request to drain packets first.

## Explanation

Encoding reports failure; the failed packet is not sent to the output writer.

## Possible causes

- FFmpeg reported the included encoding error.

## What to check

1. Check the error text and the input audio settings.

## Recommended action

Resolve the specific FFmpeg encoding failure.
