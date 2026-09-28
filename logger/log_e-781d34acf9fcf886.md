# DVR timeline interval is reversed

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_e
Log level: error
Documentation ID: log_e-781d34acf9fcf886

## Log pattern

```text
timeline start time %lu is greater than end time %lu for %s/%s
```

## Parameters

- Argument 1 — Interval start: Unsigned UTC start in milliseconds since the Unix epoch.
- Argument 2 — Interval end: Unsigned UTC end in milliseconds since the Unix epoch.
- Argument 3 — Application: Text naming the application.
- Argument 4 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
timeline start time 1700000001000 is greater than end time 1700000000000 for live/camera
```

## Short description

DVR timeline interval is reversed.

## Trigger condition

A cached timeline interval has a start later than its end.

## Explanation

The period timeline is marked for rebuilding and reconstructed from recorded segments before being returned.

## Possible causes

- A cached timeline interval has a start later than its end.

## What to check

1. Check recorded segment timestamps and recent deletion or update operations.

## Recommended action

Check recorded segment timestamps and recent deletion or update operations.
