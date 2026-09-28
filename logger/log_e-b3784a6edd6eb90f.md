# AJA card does not support the output video format

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-b3784a6edd6eb90f

## Log pattern

```text
Device '%s' doesn't support output video format '%s'
```

## Parameters

- Argument 1 — Device: Configured AJA device selector or display name, as indicated by the message.
- Argument 2 — Video format: Runtime description of the requested output video format.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Device 'AJA device' doesn't support output video format '3840x2160p25'
```

## Short description

AJA card does not support the output video format.

## Trigger condition

The card capability check rejects the selected output video format.

## Explanation

Output video setup returns failure before hardware configuration.

## Possible causes

- The card capability check rejects the selected output video format.

## What to check

1. Check the requested video format against the card capabilities.

## Recommended action

Use a video format supported by the selected card.

## Related log messages

- [AJA sender video setup failed](log_e-848c253ef8c5bf04.md)
