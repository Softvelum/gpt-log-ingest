# OMT video timestamp cannot use the video timecode reference

Product: Nimble Streamer
Component: OMT media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-baee04e13cfca09d

## Log pattern

```text
failed to get system timecode for video pts=%lld, last_video_pts=%lld for [%s]
```

## Parameters

- Argument 1 — Frame timestamp: Video timestamp in 100-nanosecond ticks.
- Argument 2 — Reference timestamp: Video timestamp linked to the SEI system-time reference, in 100-nanosecond ticks.
- Argument 3 — OMT name: Requested output name or resolved full input source name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get system timecode for video pts=200000000, last_video_pts=0 for [Output]
```

## Short description

OMT video timestamp cannot use the video timecode reference.

## Trigger condition

The video timestamp lies outside the strictly-less-than-ten-second distance accepted around the video reference.

## Explanation

The frame is skipped without failing the overall message handler. The reference value is tied to the latest SEI timecode, rather than necessarily the last video frame received.

## Possible causes

- The video timestamp lies outside the strictly-less-than-ten-second distance accepted around the video reference.

## What to check

1. Check the media timestamp distance from the video reference.

## Recommended action

Supply refreshed timecodes and timestamps within the accepted interval.

## Related log messages

- [OMT video waits for a video timecode reference](log_d-833df2b0537d2360.md)
