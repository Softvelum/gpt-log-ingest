# AJA output audio is not planar floating point

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-ae550480ee93fff1

## Log pattern

```text
AJA output audio requires fltp for %s, format=%d
```

## Parameters

- Argument 1 — Session name: Configured AJA device and channel name.
- Argument 2 — Sample format: Integer media-processing sample-format identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
AJA output audio requires fltp for 0/0, format=0
```

## Short description

AJA output audio is not planar floating point.

## Trigger condition

Audio configuration receives a sample format other than planar floating point.

## Explanation

Audio configuration returns failure before checking its sample rate and channel count.

## Possible causes

- Audio configuration receives a sample format other than planar floating point.

## What to check

1. Check the audio sample format supplied to the AJA output.

## Recommended action

Convert output audio to planar floating point.

## Related log messages

- [AJA output audio sample rate is not 48 kHz](log_e-66e3a3829f52c772.md)
