# Interleaved RTSP channel has no media-track object

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-5f9f3bec309f7ab2

## Log pattern

```text
unable to get stream info channel=%d stream=[%s]
```

## Parameters

- Argument 1 — Channel: Unsigned eight-bit interleaved channel number.
- Argument 2 — Stream identity: Application and stream label used by the RTSP parser.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unable to get stream info channel=0 stream=[live/camera]
```

## Short description

Interleaved RTSP channel has no media-track object.

## Trigger condition

An incoming channel has a known data/control type but no corresponding media-track object.

## Explanation

The complete interleaved frame is consumed and skipped; the parser returns success. Unknown channel numbers without any type mapping are silently skipped elsewhere.

## Possible causes

- An incoming channel has a known data/control type but no corresponding media-track object.

## What to check

1. Check that interleaved channel negotiation and media-track registration agree.
