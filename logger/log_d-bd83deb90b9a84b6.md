# HLS fragmented MP4 audio segment is cached

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_d
Log level: debug
Documentation ID: log_d-bd83deb90b9a84b6

## Log pattern

```text
add_audio_hls_segment app=%s, stream=%s, duration=%d,%d, size=%d
```

## Parameters

- Argument 1 — Application: Application producing the item.
- Argument 2 — Stream: Stream producing the item.
- Argument 3 — Whole seconds: Intended whole-second duration, supplied with a mismatched 64-bit argument.
- Argument 4 — Milliseconds: Intended fractional millisecond duration, supplied with a mismatched 64-bit argument.
- Argument 5 — Size: Stored item size in bytes, including encryption when applied.

## Log examples

No reliable complete example is available because the duration arguments are 64-bit while their conversions expect 32-bit values.

## Short description

HLS fragmented MP4 audio segment is cached.

## Trigger condition

The serialized audio segment is stored and marked ready in the cache.

## Explanation

The item is then attached to its live output list. Both duration values are 64-bit arguments passed to 32-bit conversions, so the printed durations are unreliable.

## Possible causes

- The serialized audio segment is stored and marked ready in the cache.

## What to check

1. Check the corresponding playlist and cache item when confirming packaging.

## Recommended action

No action is required for expected output.
