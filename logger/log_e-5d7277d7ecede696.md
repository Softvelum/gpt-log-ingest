# AJA output channel is unavailable

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-5d7277d7ecede696

## Log pattern

```text
No such output channel '%s' on '%s'
```

## Parameters

- Argument 1 — Channel name: Human-readable channel description returned by the AJA runtime.
- Argument 2 — Device: Configured AJA device selector or display name, as indicated by the message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
No such output channel 'Channel 8' on 'AJA device'
```

## Short description

AJA output channel is unavailable.

## Trigger condition

The requested output channel is invalid or unsupported by the card.

## Explanation

Output video setup returns failure before configuring format or routing.

## Possible causes

- The requested output channel is invalid or unsupported by the card.

## What to check

1. Check the selected output channel against the device channels.

## Recommended action

Select a valid supported output channel.

## Related log messages

- [AJA sender video setup failed](log_e-848c253ef8c5bf04.md)
- [AJA output channel number is invalid](log_e-b917dd3f3dfc5734.md)
