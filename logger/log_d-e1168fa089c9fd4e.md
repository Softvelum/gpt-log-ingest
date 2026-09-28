# DVB subtitle output packet identifier is assigned

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_d
Log level: debug
Documentation ID: log_d-e1168fa089c9fd4e

## Log pattern

```text
subtitle_pid=%d assigned for stream=[%s/%s] sending to %s:%d
```

## Parameters

- Argument 1 — PID: Assigned transport packet identifier. Zero can indicate allocation exhaustion.
- Argument 2 — Application: Output source application.
- Argument 3 — Stream: Output source stream.
- Argument 4 — Destination: Configured destination IP address.
- Argument 5 — Port: Configured destination port.

## Log examples

Illustrative examples; these are not captured server logs.

```text
subtitle_pid=1001 assigned for stream=[live/camera] sending to 192.0.2.20:5000
```

## Short description

DVB subtitle output packet identifier is assigned.

## Trigger condition

The DVB subtitle packet identifier is newly assigned or changed during output stream setup.

## Explanation

Continuity state is initialized for the assignment. Automatic allocation searches unused identifiers from 1000 through 2000. A zero result means identifiers were exhausted and assignment stops.

## Possible causes

- The DVB subtitle packet identifier is newly assigned or changed during output stream setup.

## What to check

1. Check assigned identifiers against the outgoing program map.

## Recommended action

No action is needed for an expected assignment.
