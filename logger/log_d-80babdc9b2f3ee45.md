# SCTE-35 splice event is retained for packaging

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_d
Log level: debug
Documentation ID: log_d-80babdc9b2f3ee45

## Log pattern

```text
[SCTE35] splice_insert: event_id=%d oon=%d immediate=%d pts=%llu duration=%llu data=%s for [%s/%s]
```

## Parameters

- Argument 1 — Event ID: SCTE-35 event identifier.
- Argument 2 — Out of network: 1 for a splice-out event or 0 for splice-in.
- Argument 3 — Immediate: 1 for an immediate event or 0 for a timed event.
- Argument 4 — PTS: Stored splice timestamp in 90 kHz ticks; an immediate event may have no specified timestamp and show 0.
- Argument 5 — Duration: Stored break duration in 90 kHz ticks.
- Argument 6 — Section bytes: Complete retained SCTE-35 section rendered as lowercase hexadecimal with a 0x prefix.
- Argument 7 — Application: Application receiving the splice metadata.
- Argument 8 — Stream: Stream receiving the splice metadata.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] splice_insert: event_id=100 oon=1 immediate=1 pts=0 duration=2700000 data=0xfc00200000000000000000000f050000006400f000002932e000000000000061b073b0 for [live/camera]
```

## Short description

SCTE-35 splice event is retained for packaging.

## Trigger condition

A supported noncancelled splice point has been parsed and is about to be added to the pending event list while debug logging is enabled.

## Explanation

Earlier matching event IDs are removed when superseded. A nonzero externally supplied duration replaces the parsed duration, so the logged duration can differ from the raw section.

## Possible causes

- A supported noncancelled splice point has been parsed and is about to be added to the pending event list while debug logging is enabled.

## What to check

1. Check the event ID, timing, and any configured duration replacement.

## Recommended action

No action is required for an expected splice event.
