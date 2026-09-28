# SDI output initialization failed

Product: Nimble Streamer
Component: SDI device output
Logging function: log_e
Log level: error
Documentation ID: log_e-e32b6af0e9cfab93

## Log pattern

```text
failed to initialize [%s/%s] error=%s
```

## Parameters

- Argument 1 — Device type: FFmpeg output device type, text.
- Argument 2 — Stream: Output stream name, text.
- Argument 3 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to initialize [decklink/live/camera] error=operation failed
```

## Short description

SDI output initialization failed.

## Trigger condition

Initializing the FFmpeg device output returns a negative result.

## Explanation

The output creation request returns failure; the converted FFmpeg error text is included.

## Possible causes

- The configured device output cannot initialize.

## What to check

1. Check the error text, selected device, video mode and audio layout.

## Recommended action

Correct the reported device configuration or availability problem and retry.
