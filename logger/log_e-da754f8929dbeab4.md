# NDI audio reader is unavailable

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-da754f8929dbeab4

## Log pattern

```text
failed to get audio ndi reader for %s
```

## Parameters

- Argument 1 — Source: NDI reader name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get audio ndi reader for camera
```

## Short description

NDI audio reader is unavailable.

## Trigger condition

Switching audio input to the named NDI source finds no reader registered under that name.

## Explanation

Audio source updating fails after the previous decoder and local reader have been reset.

## Possible causes

- The reader has not been created, was removed, or could not restart.

## What to check

1. Check the source name, reader creation messages, and NDI input availability.

## Recommended action

Restore the named input reader and restart the dependent audio pipeline.
