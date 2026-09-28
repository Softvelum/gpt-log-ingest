# Audio encoder requires reset after sample rate change

Product: Nimble Streamer
Component: Audio frame preparation
Logging function: log_e
Log level: error
Documentation ID: log_e-18ddb9fedfaf5888

## Log pattern

```text
audio encoder reset for stream='%s' as sample rate changed (%d -> %d)
```

## Parameters

- Argument 1 — Stream: Output stream name, text.
- Argument 2 — Previous value: Previous sample rate, integer samples per second.
- Argument 3 — New value: Current input sample rate, integer samples per second.

## Log examples

Illustrative examples; these are not captured server logs.

```text
audio encoder reset for stream='live/camera' as sample rate changed (44100 -> 48000)
```

## Short description

Audio encoder requires reset after sample rate change.

## Trigger condition

The current input sample rate differs from the retained encoder-frame sample rate.

## Explanation

Frame preparation returns failure so the existing encoder is no longer used for this frame. The log does not itself recreate an encoder.

## Possible causes

- The audio input changed sample rate after initialization.

## What to check

1. Compare the old and new sample rate and the upstream audio changes.

## Recommended action

Recreate the encoder with the new audio parameters or keep the input parameters stable.
