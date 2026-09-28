# Audio encoder packet retrieval failed

Product: Nimble Streamer
Component: FFmpeg audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-cdad415f2425bbb1

## Log pattern

```text
failed to recv encoded audio packet from encoder: %s
```

## Parameters

- Argument 1 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to recv encoded audio packet from encoder: operation failed
```

## Short description

Audio encoder packet retrieval failed.

## Trigger condition

Receiving an encoded audio packet returns a nonzero result other than a request for more input.

## Explanation

Encoding reports failure; the failed packet is not sent to the output writer.

## Possible causes

- FFmpeg reported the included encoding error.

## What to check

1. Check the error text and the input audio settings.

## Recommended action

Resolve the specific FFmpeg encoding failure.
