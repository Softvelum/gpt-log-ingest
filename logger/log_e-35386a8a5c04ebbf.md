# x264 preset could not be initialized

Product: Nimble Streamer
Component: x264 H.264 encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-35386a8a5c04ebbf

## Log pattern

```text
failed to get default preset
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get default preset
```

## Short description

x264 preset could not be initialized.

## Trigger condition

Applying the requested preset and tuning returns a negative result.

## Explanation

H.264 encoder initialization fails.

## Possible causes

- The selected preset or tuning cannot initialize x264 parameters.

## What to check

1. Check the video format and selected x264 profile, preset, tuning, and codec options.

## Recommended action

Correct the rejected encoder configuration.
