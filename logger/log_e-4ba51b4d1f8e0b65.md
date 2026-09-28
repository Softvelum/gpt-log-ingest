# NDI video timestamp is too far from its timecode reference

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-4ba51b4d1f8e0b65

## Log pattern

```text
failed to get system timecode for video pts=%lld, last_video_pts=%lld, ndi_name="%s"
```

## Parameters

- Argument 1 — Frame timestamp: Video presentation timestamp interpreted in 100-nanosecond ticks.
- Argument 2 — Reference timestamp: Video presentation timestamp associated with the latest SEI system-time reference, in 100-nanosecond ticks.
- Argument 3 — NDI name: Requested or resolved NDI source name identifying the affected input or output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get system timecode for video pts=200000000, last_video_pts=0, ndi_name="HOST (Output)"
```

## Short description

NDI video timestamp is too far from its timecode reference.

## Trigger condition

A video timestamp cannot be mapped to the established system-time reference; the accepted absolute timestamp distance is strictly less than ten seconds.

## Explanation

The frame is skipped and processing returns success. The logged reference timestamp is the reference video timestamp, not necessarily the most recently received video frame.

## Possible causes

- A video timestamp cannot be mapped to the established system-time reference; the accepted absolute timestamp distance is strictly less than ten seconds.

## What to check

1. Check the video timestamp distance from the SEI reference and the frequency of refreshed timecodes.

## Recommended action

Provide a current video timecode reference with media timestamps within ten seconds.

## Related log messages

- [NDI video waits for a video timecode reference](log_d-5182c47e615b972b.md)
