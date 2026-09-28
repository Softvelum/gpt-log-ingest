# x264 option name is invalid

Product: Nimble Streamer
Component: x264 H.264 encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-502baab6bf64bd92

## Log pattern

```text
bad param name: %s
```

## Parameters

- Argument 1 — Option: Requested option name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
bad param name: unknown-option
```

## Short description

x264 option name is invalid.

## Trigger condition

The x264 option parser reports an unknown parameter name.

## Explanation

H.264 encoder initialization fails.

## Possible causes

- The option is not accepted by x264.

## What to check

1. Check the video format and selected x264 profile, preset, tuning, and codec options.

## Recommended action

Correct the rejected encoder configuration.
