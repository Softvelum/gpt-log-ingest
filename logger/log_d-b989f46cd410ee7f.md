# SCTE-35 splice point is queued for streaming output

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_d
Log level: debug
Documentation ID: log_d-b989f46cd410ee7f

## Log pattern

```text
[SCTE35] splice_insert: event_id=%d oon=%d immediate=%d pts=%llu actual_pts=%llu duration=%llu data=%s for [%s/%s]
```

## Parameters

- Argument 1 — Event ID: SCTE-35 splice event identifier.
- Argument 2 — Out of network: 1 for splice out, 0 for splice in.
- Argument 3 — Immediate: 1 when the command takes effect at its associated media sample.
- Argument 4 — PTS: Parsed SCTE-35 timestamp in 90 kHz ticks; zero for this immediate example.
- Argument 5 — Actual PTS: Effective marker timestamp in 90 kHz ticks.
- Argument 6 — Duration: Advertised break duration in 90 kHz ticks.
- Argument 7 — Data: Complete SCTE-35 section as hexadecimal text.
- Argument 8 — Application: Application containing the live stream.
- Argument 9 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] splice_insert: event_id=100 oon=1 immediate=1 pts=0 actual_pts=90000 duration=2700000 data=0xfc00200000000000000000000f050000006400f000002932e000000000000061b073b0 for [live/camera]
```

## Short description

SCTE-35 splice point is queued for streaming output.

## Trigger condition

Debug logging is enabled and a parsed, uncancelled supported marker is accepted for HLS or enabled DASH ad signaling.

## Explanation

The marker is queued for applicable outputs. Immediate markers use the associated media timestamp; scheduled markers use their own timestamp.

## Possible causes

- Debug logging is enabled and a parsed, uncancelled supported marker is accepted for HLS or enabled DASH ad signaling.

## What to check

1. Compare the event, immediate flag, raw section, and effective media time when checking ad alignment.

## Recommended action

No action is needed for expected ad markers.
