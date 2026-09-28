# SCTE-35 packet no longer has an accepted type

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-fc8edfb3c216a35c

## Log pattern

```text
updateAvailable scte35 type changed for actual_scte35_pid=%d to %d
```

## Parameters

- Argument 1 — packet\_id: MPEG-TS packet identifier printed as a decimal integer.
- Argument 2 — stream\_type: Numeric media stream-type code printed in decimal; this is not a packet identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable scte35 type changed for actual_scte35_pid=300 to 27
```

## Short description

SCTE-35 packet no longer has an accepted type.

## Trigger condition

SCTE-35 processing is enabled and the active packet still equals the default, but its advertised type is no longer 134 (SCTE-35).

## Explanation

The active identifier is cleared and is not added to the requested packet set by this branch.
