# H.264 encoder could not open

Product: Nimble Streamer
Component: x264 H.264 encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-3ea9d27be6bc6b6a

## Log pattern

```text
failed to open the encoder
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open the encoder
```

## Short description

H.264 encoder could not open.

## Trigger condition

Opening x264 with the prepared parameters returns no encoder.

## Explanation

H.264 encoder initialization fails.

## Possible causes

- x264 cannot open the selected encoding configuration.

## What to check

1. Check the video format and selected x264 profile, preset, tuning, and codec options.

## Recommended action

Correct the rejected encoder configuration.
