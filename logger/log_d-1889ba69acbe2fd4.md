# Fragmented MP4 HLS audio needs a discontinuity

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_d
Log level: debug
Documentation ID: log_d-1889ba69acbe2fd4

## Log pattern

```text
resync hls fmp4 audio with video, current_timestamp=%lld but expected_timestamp=%lld for [%s/%s]
```

## Parameters

- Argument 1 — Current timestamp: New audio timestamp in audio-timescale ticks.
- Argument 2 — Expected timestamp: Previous timestamp plus 1024 samples converted into audio-timescale ticks.
- Argument 3 — Application: Application containing the live stream.
- Argument 4 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
resync hls fmp4 audio with video, current_timestamp=1100 but expected_timestamp=1021 for [live/camera]
```

## Short description

Fragmented MP4 HLS audio needs a discontinuity.

## Trigger condition

AAC audio synchronization is enabled, no discontinuity is already set, and the new timestamp exceeds the previous timestamp plus one AAC frame and the allowed precision error.

## Explanation

The HLS fragmented MP4 output marks a discontinuity and continues adding audio.

## Possible causes

- AAC audio synchronization is enabled, no discontinuity is already set, and the new timestamp exceeds the previous timestamp plus one AAC frame and the allowed precision error.

## What to check

1. Compare input timestamps with the AAC sample rate and 1024-sample frame duration.

## Recommended action

Investigate unexpected gaps in source audio timing.
