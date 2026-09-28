# DASH subtitle data trails the current media segment

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_i
Log level: info
Documentation ID: log_i-041ce9c78fdf885a

## Log pattern

```text
dash subtitle delay=%lld for [%s/%s]
```

## Parameters

- Argument 1 — Delay: Difference between media-segment start and subtitle-segment start, in milliseconds.
- Argument 2 — Application: Application containing the live stream.
- Argument 3 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
dash subtitle delay=16000 for [live/camera]
```

## Short description

DASH subtitle data trails the current media segment.

## Trigger condition

A nonempty subtitle segment starts more than 15000 milliseconds before the current DASH media segment.

## Explanation

The subtitle segment is still submitted and its text buffer is cleared.

## Possible causes

- A nonempty subtitle segment starts more than 15000 milliseconds before the current DASH media segment.

## What to check

1. Compare subtitle timestamps with audio/video segment timestamps.

## Recommended action

Correct unexpected subtitle timestamp delay.
