# Audio encoder could not be opened

Product: Nimble Streamer
Component: FFmpeg audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-e2048deb4033eaa4

## Log pattern

```text
failed to open %s audio encoder: %s
```

## Parameters

- Argument 1 — Encoder: Requested or selected audio encoder name, text.
- Argument 2 — Error text: Diagnostic description supplied by the operating system or library, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open aac audio encoder: operation failed
```

## Short description

Audio encoder could not be opened.

## Trigger condition

Opening the selected FFmpeg audio encoder with the supplied audio settings returns a negative result.

## Explanation

Its context is freed and initialization fails.

## Possible causes

- The selected encoder rejects the configured sample format, rate, layout, or other codec options.

## What to check

1. Check the encoder name, error text, and audio settings.

## Recommended action

Correct the settings identified by the FFmpeg diagnostic.
