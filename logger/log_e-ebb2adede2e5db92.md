# AAC input channel layout is unsupported

Product: Nimble Streamer
Component: FDK AAC audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ebb2adede2e5db92

## Log pattern

```text
unsupported channel_layout='%s'
```

## Parameters

- Argument 1 — Layout: Text description of the unsupported input channel layout.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported channel_layout='9 channels'
```

## Short description

AAC input channel layout is unsupported.

## Trigger condition

The input channel layout cannot be mapped to an FDK channel mode.

## Explanation

Encoder initialization stops before setting the channel mode. One diagnostic prints a layout description; the other prints order and count when description fails.

## Possible causes

- The input arrangement is outside the supported mapping.

## What to check

1. Check the channel count and arrangement. The mapping covers 1 through 6 channels and optional 8-channel layouts when the library build enables them.

## Recommended action

Convert audio to a supported channel layout.

## Related log messages

- [AAC input channel layout is unsupported](log_e-5420e5d749991891.md)
