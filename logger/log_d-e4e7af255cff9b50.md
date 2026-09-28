# Playback detects an advertising splice-out point

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-e4e7af255cff9b50

## Log pattern

```text
s=%d: splice_out_point, event_id=%d, stream_time=%lld
```

## Parameters

- Argument 1 — Socket: Connection descriptor reported by the playback transport.
- Argument 2 — Event ID: SCTE-35 splice event identifier, displayed as a decimal integer.
- Argument 3 — Stream time: Mapped playback scheduling time in milliseconds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42: splice_out_point, event_id=100, stream_time=60000
```

## Short description

Playback detects an advertising splice-out point.

## Trigger condition

SLDP processes a noncancelled splice-out point before requesting advertising rules, or RTMP processes a noncancelled out-of-network splice-insert event.

## Explanation

The event time is used to select SCTE-35 advertising rules and content. An immediate event uses the current playback time; a timed event is mapped from its media timestamp. The log does not confirm that any advertisement is available.

## Possible causes

- SLDP processes a noncancelled splice-out point before requesting advertising rules, or RTMP processes a noncancelled out-of-network splice-insert event.

## What to check

1. Check the event ID, stream time, and matching advertising rules.

## Recommended action

No action is required for an expected splice-out event.
