# DVR export video timescale changed

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-80fbe67cb63ae579

## Log pattern

```text
video timescale changed period=%d stream=%s utc_start=%lld utc_end=%lld (%u -> %u)
```

## Parameters

- Argument 1 — Recording period: Integer identifier of the continuous recording period.
- Argument 2 — Stream: Text naming the application and stream.
- Argument 3 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 4 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 5 — Previous timescale: Unsigned video clock ticks per second.
- Argument 6 — New timescale: Unsigned video clock ticks per second.

## Log examples

Illustrative examples; these are not captured server logs.

```text
video timescale changed period=1 stream=live/camera utc_start=1700000000 utc_end=1700000060 (90000 -> 1000)
```

## Short description

DVR export video timescale changed.

## Trigger condition

A video fragment uses a different timescale from earlier selected fragments.

## Explanation

The export cannot combine these fragments and closes.

## Possible causes

- A video fragment uses a different timescale from earlier selected fragments.

## What to check

1. Export a range with a consistent video timescale or correct recording timestamp configuration.

## Recommended action

Export a range with a consistent video timescale or correct recording timestamp configuration.
