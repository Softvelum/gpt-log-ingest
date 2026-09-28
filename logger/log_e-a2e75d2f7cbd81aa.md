# AV1 hardware decoder profile is unknown

Product: Nimble Streamer
Component: FFmpeg video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-a2e75d2f7cbd81aa

## Log pattern

```text
unknown av1 profile=%d for [%s]
```

## Parameters

- Argument 1 — Profile: AV1 configuration profile number, integer.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown av1 profile=3 for [live/camera]
```

## Short description

AV1 hardware decoder profile is unknown.

## Trigger condition

In an MA35D-enabled build, AV1 configuration for the hardware decoder has a profile greater than two.

## Explanation

The error is logged while decoder setup continues; a valid bit depth has not been selected by this branch.

## Possible causes

- The AV1 profile is outside the handled profiles zero through two.

## What to check

1. Check the AV1 configuration profile and downstream decoder-open outcome.

## Recommended action

Provide AV1 input with a supported profile.
