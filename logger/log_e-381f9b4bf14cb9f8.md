# Codec option name is unknown

Product: Nimble Streamer
Component: FFmpeg logging and media input
Logging function: log_e
Log level: error
Documentation ID: log_e-381f9b4bf14cb9f8

## Log pattern

```text
invalid param name: %s=%s for [%s]
```

## Parameters

- Argument 1 — Option: Requested codec option name, text.
- Argument 2 — Value: Requested codec option value, text.
- Argument 3 — Stream: Stream name associated with the encoder or decoder being configured, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid param name: example_option=invalid for [live/camera]
```

## Short description

Codec option name is unknown.

## Trigger condition

While applying encoder or decoder options, the option name is not recognized by either the codec-specific or common codec settings.

## Explanation

The rejected option is logged and processing continues with subsequent options; this helper does not abort codec initialization.

## Possible causes

- Neither codec-specific nor common settings recognize the option name.

## What to check

1. Check the option spelling and whether the selected codec supports that setting.

## Recommended action

Use a supported option and value.

## Related log messages

- [Codec option value is out of range](log_e-20f17c3a955fb6cb.md)
- [Codec option could not be applied](log_e-39096a7821198159.md)
- [Codec option value is invalid](log_e-47911d99fe9cd1a2.md)
