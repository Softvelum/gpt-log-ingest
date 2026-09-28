# AJA capture channel is unavailable

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-81464fc7bc12bb1b

## Log pattern

```text
No such channel '%s' on '%s'
```

## Parameters

- Argument 1 — Channel name: Human-readable channel description returned by the AJA runtime.
- Argument 2 — Device: Configured AJA device selector or display name, as indicated by the message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
No such channel 'Channel 8' on 'AJA device'
```

## Short description

AJA capture channel is unavailable.

## Trigger condition

After deriving a channel from the input source or applying the default, the card reports that the channel is unsupported.

## Explanation

Input source/channel selection returns failure before video setup.

## Possible causes

- After deriving a channel from the input source or applying the default, the card reports that the channel is unsupported.

## What to check

1. Check that the requested capture channel exists on the selected card.

## Recommended action

Select a channel supported by that card.

## Related log messages

- [AJA capture input selection failed](log_e-11855c615785c315.md)
