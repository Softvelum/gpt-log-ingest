# DVR segment cannot be read for replacement

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_e
Log level: error
Documentation ID: log_e-1f120853e6fc4dbf

## Log pattern

```text
failed to load dvr segment utc_time=%lld start_time=%lld duration=%lld period=%d
```

## Parameters

- Argument 1 — UTC time: 64-bit segment UTC time in milliseconds since the Unix epoch.
- Argument 2 — Media start: 64-bit media start timestamp in the segment track timescale.
- Argument 3 — Media duration: 64-bit duration in the segment track timescale.
- Argument 4 — Period: Unsigned recording period identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load dvr segment utc_time=1700000000000 start_time=90000 duration=540000 period=1
```

## Short description

DVR segment cannot be read for replacement.

## Trigger condition

The media file cannot be opened for a video blackout or MP3 mute operation.

## Explanation

The current replacement operation returns failure.

## Possible causes

- The media file cannot be opened for a video blackout or MP3 mute operation.

## What to check

1. Check recording-file availability, access, and indexed byte ranges.

## Recommended action

Check recording-file availability, access, and indexed byte ranges.
