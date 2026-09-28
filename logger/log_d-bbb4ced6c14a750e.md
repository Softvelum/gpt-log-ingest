# DVR AAC segment muting starts

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_d
Log level: debug
Documentation ID: log_d-bbb4ced6c14a750e

## Log pattern

```text
mute audio segment utc_time=%lld start_time=%lld duration=%lld period=%d
```

## Parameters

- Argument 1 — UTC time: 64-bit segment UTC time in milliseconds since the Unix epoch.
- Argument 2 — Media start: 64-bit media start timestamp in the segment track timescale.
- Argument 3 — Media duration: 64-bit duration in the segment track timescale.
- Argument 4 — Period: Unsigned recording period identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
mute audio segment utc_time=1700000000000 start_time=90000 duration=540000 period=1
```

## Short description

DVR AAC segment muting starts.

## Trigger condition

An audio cleanup operation selects an AAC segment for muting.

## Explanation

A matching silence sample is requested and a replacement fragment is built.

## Possible causes

- An audio cleanup operation selects an AAC segment for muting.

## What to check

1. Check subsequent muting errors if audio remains present.

## Recommended action

Check subsequent muting errors if audio remains present.
