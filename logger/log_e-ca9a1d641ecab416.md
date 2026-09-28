# Time-aligned keyframe interval has no positive ticks

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_e
Log level: error
Documentation ID: log_e-ca9a1d641ecab416

## Log pattern

```text
invalid timebase=%d/%d, failed to encode video frame for [%s/%s]
```

## Parameters

- Argument 1 — Numerator: Input time-base numerator, integer seconds-per-tick rational numerator.
- Argument 2 — Denominator: Input time-base denominator, integer seconds-per-tick rational denominator.
- Argument 3 — Application: Output application name, text.
- Argument 4 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid timebase=1/25, failed to encode video frame for [live/encoded]
```

## Short description

Time-aligned keyframe interval has no positive ticks.

## Trigger condition

Converting the configured millisecond keyframe interval into the input frame time base yields zero or a negative value.

## Explanation

The current frame is skipped and the operation returns success, so the worker continues despite the error wording.

## Possible causes

- The interval is nonpositive or too small for the supplied time base, or the time base is invalid.

## What to check

1. Check both the keyframe boundary in milliseconds and the printed seconds-per-tick rational.

## Recommended action

Use a valid time base and an interval that converts to at least one tick.
