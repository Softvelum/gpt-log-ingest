# FFmpeg encoder name cannot be resolved

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_e
Log level: error
Documentation ID: log_e-41d8578b1fca2eeb

## Log pattern

```text
failed to get FFmpeg encoder='%s'
```

## Parameters

- Argument 1 — Encoder: Requested FFmpeg encoder name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get FFmpeg encoder='missing_encoder'
```

## Short description

FFmpeg encoder name cannot be resolved.

## Trigger condition

While initializing output from its first encoded packet, FFmpeg lookup by the configured encoder name returns no codec identifier.

## Explanation

The error is reported but the function continues matching explicit codec names; later output setup may fail if no supported codec is selected.

## Possible causes

- The configured encoder name is absent from the loaded FFmpeg codec library.

## What to check

1. Check the exact encoder name and codec-library availability.

## Recommended action

Select an encoder name registered by the loaded library.
