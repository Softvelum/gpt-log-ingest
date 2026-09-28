# NDI video reader is unavailable

Product: Nimble Streamer
Component: Video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-0cfef03b3f72ba1f

## Log pattern

```text
failed to get video ndi reader for %s
```

## Parameters

- Argument 1 — Source: NDI input reader name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get video ndi reader for camera
```

## Short description

NDI video reader is unavailable.

## Trigger condition

Switching video input to the named NDI source finds no registered reader under that name.

## Explanation

Video source updating fails after resetting the previous decoder and local reader.

## Possible causes

- The reader has not been created, was removed, or could not restart.

## What to check

1. Check the source name, reader creation messages, and NDI input availability.

## Recommended action

Restore the named reader and restart the dependent video pipeline.
