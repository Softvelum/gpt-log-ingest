# AJA video reader is unavailable

Product: Nimble Streamer
Component: Video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-0a3d541fca6b79a2

## Log pattern

```text
failed to get video aja reader for %s
```

## Parameters

- Argument 1 — Source: AJA input reader name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get video aja reader for camera
```

## Short description

AJA video reader is unavailable.

## Trigger condition

Switching video input to the named AJA source finds no registered reader under that name.

## Explanation

Video source updating fails after resetting the previous decoder and local reader.

## Possible causes

- The reader has not been created, was removed, or could not restart. The AJA helper process may also be unavailable.

## What to check

1. Check the source name, reader creation messages, and AJA input availability.

## Recommended action

Restore the named reader and restart the dependent video pipeline.
