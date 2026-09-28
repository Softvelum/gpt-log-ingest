# DASH segment timeline diverges from wall-clock time

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_i
Log level: info
Documentation ID: log_i-c2b4f2e0aa95348a

## Log pattern

```text
%s check dash availability delta=%lld exceeds max segment diff limit=%lld for [%s/%s]
```

## Parameters

- Argument 1 — Media: video or audio track triggering the reset.
- Argument 2 — Difference: Absolute segment-time versus UTC difference in milliseconds.
- Argument 3 — Limit: Maximum allowed difference in milliseconds, derived from configured chunk count times chunk duration.
- Argument 4 — Application: Application containing the affected live stream.
- Argument 5 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
video check dash availability delta=65000 exceeds max segment diff limit=60000 for [live/camera]
```

## Short description

DASH segment timeline diverges from wall-clock time.

## Trigger condition

The absolute difference between calculated segment availability time and current UTC time exceeds the configured maximum segment-window duration.

## Explanation

The DASH segment list is reset, the current track timestamp is reinitialized, and availability/publish times become unset until a later segment. Enabled presentation-time offsets are also updated. The difference can be ahead of or behind wall-clock time.

## Possible causes

- The absolute difference between calculated segment availability time and current UTC time exceeds the configured maximum segment-window duration.

## What to check

1. Check timestamp pacing, clock changes, and the configured chunk count and duration.

## Recommended action

Correct source timing or clock discontinuities causing repeated resets.
