# PES fixed marker bits are invalid

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-d5091ab3dc36f5ce

## Log pattern

```text
table_id not PES pid=%d, t_id=%d
```

## Parameters

- Argument 1 — PID: Affected elementary-stream packet identifier.
- Argument 2 — Marker: Two-bit value found where binary 10 is required.

## Log examples

Illustrative examples; these are not captured server logs.

```text
table_id not PES pid=256, t_id=0
```

## Short description

PES fixed marker bits are invalid.

## Trigger condition

The two fixed marker bits following the PES length are not binary 10.

## Explanation

Despite the label table_id, the printed value is this two-bit PES marker. The current transport packet is rejected. Its elementary-stream parser is reset, buffered partial PES data is discarded, and the camera parser searches for another packet boundary.

## Possible causes

- The two fixed marker bits following the PES length are not binary 10.

## What to check

1. Check PES framing and the marker bits after the length field.

## Recommended action

Correct the source PES header.
