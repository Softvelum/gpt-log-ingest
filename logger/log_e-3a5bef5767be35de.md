# MP3 sample count does not match timestamp accounting

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-3a5bef5767be35de

## Log pattern

```text
failed to get pts, frame_size=%d mpa_sample_count=%d
```

## Parameters

- Argument 1 — Encoder frame size: Samples per encoder frame, integer.
- Argument 2 — Header sample count: Samples derived from the encoded MPEG-audio duration, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get pts, frame_size=1152 mpa_sample_count=576
```

## Short description

MP3 sample count does not match timestamp accounting.

## Trigger condition

The sample count calculated from the encoded header differs from the encoder frame size.

## Explanation

The packet is rejected before a timestamp is taken from the queue.

## Possible causes

- The encoded MPEG-audio duration disagrees with the encoder frame size.

## What to check

1. Compare the two sample counts and selected sample rate.

## Recommended action

Resolve the indicated MP3 encoding or frame-accounting failure.
