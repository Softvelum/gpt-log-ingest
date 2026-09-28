# Decoded AAC stream information is unavailable

Product: Nimble Streamer
Component: FDK AAC audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-26a0d2f0bec4a17a

## Log pattern

```text
failed to get stream info [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get stream info [live/camera]
```

## Short description

Decoded AAC stream information is unavailable.

## Trigger condition

In an FDK-enabled build, a decoded frame is ready but the library returns no stream information.

## Explanation

Packet processing fails before creating an output frame.

## Possible causes

- The decoder or sample-handling operation described by the message failed.

## What to check

1. Check the AAC input parameters and accompanying decoder diagnostics.

## Recommended action

Resolve the indicated AAC decode or output-frame failure.
