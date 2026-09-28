# DVR export video segments overlap

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-dcf1f3d33118a79f

## Log pattern

```text
overlapped video segment period=%d start_time=%d time=%lld period=%d stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Recording period: Integer identifier of the continuous recording period.
- Argument 2 — Segment start: 64-bit segment start timestamp in that track's media timescale.
- Argument 3 — Overlap: 64-bit duration of overlap in milliseconds.
- Argument 4 — Recording period: Integer identifier of the continuous recording period.
- Argument 5 — Stream: Text naming the application and stream.
- Argument 6 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 7 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

The segment start is a 64-bit value supplied to a 32-bit integer conversion.

## Log examples

No sound example is given because the segment-start argument does not match its printf conversion.

## Short description

DVR export video segments overlap.

## Trigger condition

A video segment starts before the end computed for the preceding segment in the same period.

## Explanation

The overlap is logged, but parsing continues.

## Possible causes

- A video segment starts before the end computed for the preceding segment in the same period.

## What to check

1. Inspect recorded video timestamps and period boundaries.

## Recommended action

Inspect recorded video timestamps and period boundaries.
