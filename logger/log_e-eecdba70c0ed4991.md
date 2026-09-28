# AJA device is not ready

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-eecdba70c0ed4991

## Log pattern

```text
Device '%s' not ready
```

## Parameters

- Argument 1 — Device: Configured AJA device selector or display name, as indicated by the message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Device 'AJA device' not ready
```

## Short description

AJA device is not ready.

## Trigger condition

A device is found but its readiness check returns false.

## Explanation

Card initialization stops before capability discovery and exclusive stream acquisition.

## Possible causes

- A device is found but its readiness check returns false.

## What to check

1. Check the selected device readiness.

## Related log messages

- [Requested AJA device was not found](log_e-62a56fcbc8821506.md)
- [AJA stream acquisition failed](log_e-ce590b9d16174df9.md)
