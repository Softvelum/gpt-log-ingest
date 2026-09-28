# AJA sender requires an output video format

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-1ccdce2219b48b54

## Log pattern

```text
Bad output video format for device '%s': video_format is required
```

## Parameters

- Argument 1 — Device: Configured AJA device selector or display name, as indicated by the message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Bad output video format for device '0': video_format is required
```

## Short description

AJA sender requires an output video format.

## Trigger condition

Output parameter validation sees an unknown video format.

## Explanation

Parameter validation fails before sender hardware setup.

## Possible causes

- Output parameter validation sees an unknown video format.

## What to check

1. Check whether output format inference succeeded.

## Recommended action

Provide video with an inferable supported output format.

## Related log messages

- [AJA output video format is missing](log_e-26f1db2d56a7472d.md)
- [AJA output format could not be inferred](log_e-323505df4d2b55aa.md)
