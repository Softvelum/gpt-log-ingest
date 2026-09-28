# H.264 sample aspect ratio changed

Product: Nimble Streamer
Component: x264 H.264 encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-3b54cde964a22cff

## Log pattern

```text
failed to change sample_aspect_ratio from %d/%d to %d/%d on the fly for [%s]
```

## Parameters

- Argument 1 — Old numerator: Integer component of the old numerator.
- Argument 2 — Old denominator: Integer component of the old denominator.
- Argument 3 — New numerator: Integer component of the new numerator.
- Argument 4 — New denominator: Integer component of the new denominator.
- Argument 5 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to change sample_aspect_ratio from 1/1 to 4/3 on the fly for [live/camera]
```

## Short description

H.264 sample aspect ratio changed.

## Trigger condition

Preserving sample aspect ratio is enabled and a new frame has a different ratio from initialization.

## Explanation

The frame write fails instead of changing the active encoder aspect ratio.

## Possible causes

- The input pixel aspect ratio changed after encoder initialization.

## What to check

1. Compare the old and new sample aspect ratios.

## Recommended action

Keep the ratio stable or recreate the encoder.
