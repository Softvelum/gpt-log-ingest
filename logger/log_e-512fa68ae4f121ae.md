# MP3 frame encoding failed

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-512fa68ae4f121ae

## Log pattern

```text
failed to encode audio, status=%d
```

## Parameters

- Argument 1 — Result: Negative LAME encoding return value, signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to encode audio, status=-1
```

## Short description

MP3 frame encoding failed.

## Trigger condition

LAME returns a negative result instead of encoded byte count.

## Explanation

The frame write fails; no new output bytes are committed to the buffer.

## Possible causes

- LAME rejected the supplied frame or output buffer.

## What to check

1. Check the negative encoder result and audio frame parameters.

## Recommended action

Resolve the indicated MP3 encoding or frame-accounting failure.
