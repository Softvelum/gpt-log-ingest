# HLS chunk retains a SCTE-35 splice event

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_d
Log level: debug
Documentation ID: log_d-8abe5c8b75cc45ff

## Log pattern

```text
[SCTE35] splice_insert: event_id=%d oon=%d immediate=%d pts=%llu duration=%llu data=%s for hls chunk=%s
```

## Parameters

- Argument 1 — Event ID: SCTE-35 event identifier.
- Argument 2 — Out of network: 1 for a splice-out event or 0 for splice-in.
- Argument 3 — Immediate: 1 for an immediate event or 0 for a timed event.
- Argument 4 — PTS: Stored splice timestamp in 90 kHz ticks; an immediate event may have no specified timestamp and show 0.
- Argument 5 — Duration: Stored break duration in 90 kHz ticks.
- Argument 6 — Section bytes: Complete retained SCTE-35 section rendered as lowercase hexadecimal with a 0x prefix.
- Argument 7 — Cache key: Application/stream path and generated segment or part name used as the cache identity.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] splice_insert: event_id=100 oon=1 immediate=1 pts=0 duration=2700000 data=0xfc00200000000000000000000f050000006400f000002932e000000000000061b073b0 for hls chunk=/live/camera/l_100_60000_10.ts
```

## Short description

HLS chunk retains a SCTE-35 splice event.

## Trigger condition

A successfully cached chunk contains a splice-in or splice-out command while debug logging is enabled.

## Explanation

The event is logged before the chunk retains its splice commands for playlist marker generation. This is not proof that a player acted on the marker.

## Possible causes

- A successfully cached chunk contains a splice-in or splice-out command while debug logging is enabled.

## What to check

1. Check the event timing, break duration, and generated chunk identity.

## Recommended action

No action is required for an expected splice event.
