# AJA output audio parameters changed

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-5fe7ff6ae67d96c8

## Log pattern

```text
reset aja output. audio params changed [%s]
```

## Parameters

- Argument 1 — Session name: Configured AJA device and channel name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
reset aja output. audio params changed [0/0]
```

## Short description

AJA output audio parameters changed.

## Trigger condition

After audio configuration, an incoming frame changes sample rate or the configured effective channel count.

## Explanation

Audio message processing returns failure. First audio arriving after video-only startup uses a separate sender-recreation path and does not emit this message.

## Possible causes

- After audio configuration, an incoming frame changes sample rate or the configured effective channel count.

## What to check

1. Check the audio rate and effective channel count before and after the change.

## Recommended action

Reinitialize output for the new supported audio parameters.

## Related log messages

- [AJA output audio sample rate is not 48 kHz](log_e-66e3a3829f52c772.md)
