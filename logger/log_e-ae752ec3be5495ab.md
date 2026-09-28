# Media input could not be opened

Product: Nimble Streamer
Component: FFmpeg logging and media input
Logging function: log_e
Log level: error
Documentation ID: log_e-ae752ec3be5495ab

## Log pattern

```text
failed to open stream %s
```

## Parameters

- Argument 1 — Input: Input media name or path, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open stream image.png
```

## Short description

Media input could not be opened.

## Trigger condition

FFmpeg cannot open the configured input.

## Explanation

The media reader reports failure.

## Possible causes

- The requested input cannot be opened by FFmpeg.

## What to check

1. Check input availability and FFmpeg diagnostics.

## Recommended action

Correct the input or requested media setting and retry.
