# AJA output channel has no required color converter

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-cbbb062d95015eee

## Log pattern

```text
Device '%s' has no CSC for output channel '%s'
```

## Parameters

- Argument 1 — Device: Configured AJA device selector or display name, as indicated by the message.
- Argument 2 — Channel name: Human-readable channel description returned by the AJA runtime.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Device 'AJA device' has no CSC for output channel 'Channel 8'
```

## Short description

AJA output channel has no required color converter.

## Trigger condition

An RGB frame-buffer format is selected but the channel index is outside the available color-space-converter count.

## Explanation

Output video setup stops because its RGB-to-output routing requires a converter for that channel.

## Possible causes

- An RGB frame-buffer format is selected but the channel index is outside the available color-space-converter count.

## What to check

1. Check the channel color-converter availability and selected pixel format.

## Recommended action

Use a channel with the required converter or an accepted non-RGB format.

## Related log messages

- [AJA sender video setup failed](log_e-848c253ef8c5bf04.md)
- [AJA output signal routing failed](log_e-bffb125b4260f371.md)
