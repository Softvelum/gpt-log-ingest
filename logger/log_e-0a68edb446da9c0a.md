# AAC frame decode failed

Product: Nimble Streamer
Component: FDK AAC audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-0a68edb446da9c0a

## Log pattern

```text
failed to decode frame [%s] e=%x
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — Library status: Library-specific status code, integer; not an operating system error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to decode frame [live/camera] e=1
```

## Short description

AAC frame decode failed.

## Trigger condition

In an FDK-enabled build, fDK returns an error other than insufficient input bits while decoding.

## Explanation

This packet operation returns success after logging, allowing later packets to continue; the failing frame is not forwarded.

## Possible causes

- The decoder or sample-handling operation described by the message failed.

## What to check

1. Check the AAC input parameters and accompanying decoder diagnostics.

## Recommended action

Resolve the indicated AAC decode or output-frame failure.
