# Codec option value is out of range

Product: Nimble Streamer
Component: FFmpeg logging and media input
Logging function: log_e
Log level: error
Documentation ID: log_e-20f17c3a955fb6cb

## Log pattern

```text
invalid param range: %s=%s for [%s]
```

## Parameters

- Argument 1 — Option: Requested codec option name, text.
- Argument 2 — Value: Requested codec option value, text.
- Argument 3 — Stream: Stream name associated with the encoder or decoder being configured, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid param range: example_option=invalid for [live/camera]
```

## Short description

Codec option value is out of range.

## Trigger condition

While applying encoder or decoder options, the library rejects the value as out of range.

## Explanation

The rejected option is logged and processing continues with subsequent options; this helper does not abort codec initialization.

## Possible causes

- The option value is outside the range accepted by the selected codec setting.

## What to check

1. Check the supported numeric range for the selected codec option.

## Recommended action

Use a supported option and value.

## Related log messages

- [Codec option name is unknown](log_e-381f9b4bf14cb9f8.md)
- [Codec option could not be applied](log_e-39096a7821198159.md)
- [Codec option value is invalid](log_e-47911d99fe9cd1a2.md)
