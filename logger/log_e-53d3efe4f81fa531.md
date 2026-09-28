# Reassembled video unit could not be accepted

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-53d3efe4f81fa531

## Log pattern

```text
failed to add assembled nal unit
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to add assembled nal unit
```

## Short description

Reassembled video unit could not be accepted.

## Trigger condition

A nonempty H.264 or H.265 fragment assembly fails when added to the video frame buffer.

## Explanation

The existing parse or stream failure is propagated. H.265 clears its fragment assembly buffer before returning the failure; H.264 returns before its caller performs the normal successful reset.

## Possible causes

- A nonempty H.264 or H.265 fragment assembly fails when added to the video frame buffer.

## What to check

1. Check the assembled video data, frame buffer capacity, and adjacent codec initialization errors.

## Related log messages

- [First video frame could not initialize its codec](log_e-c8e7a2b856d78d09.md)
