# AJA output channel number is invalid

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-b917dd3f3dfc5734

## Log pattern

```text
Bad AJA output channel number '%s' for device '%s'
```

## Parameters

- Argument 1 — Channel text: Original channel portion of the configured name.
- Argument 2 — Device: Configured AJA device selector or display name, as indicated by the message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Bad AJA output channel number '99' for device '0'
```

## Short description

AJA output channel number is invalid.

## Trigger condition

The channel portion of the device/channel name parses to an invalid AJA channel value.

## Explanation

Parameter construction returns failure before acquiring or configuring the card. Unparseable numeric text uses the default channel value before validation.

## Possible causes

- The channel portion of the device/channel name parses to an invalid AJA channel value.

## What to check

1. Check the numeric channel portion of the configured device/channel name.

## Recommended action

Select a valid AJA channel identifier.
