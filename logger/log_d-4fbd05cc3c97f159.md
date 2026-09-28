# PES optional header extends beyond the first packet

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-4fbd05cc3c97f159

## Log pattern

```text
pes header must be in first block entirely pid=%d
```

## Parameters

- Argument 1 — PID: Affected elementary-stream packet identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
pes header must be in first block entirely pid=256
```

## Short description

PES optional header extends beyond the first packet.

## Trigger condition

Skipping the remaining declared optional PES header would move beyond the end of the 188-byte payload-start packet.

## Explanation

The current transport packet is rejected. Its elementary-stream parser is reset, buffered partial PES data is discarded, and the camera parser searches for another packet boundary.

## Possible causes

- Skipping the remaining declared optional PES header would move beyond the end of the 188-byte payload-start packet.

## What to check

1. Check optional-header length and packet layout.

## Recommended action

Correct malformed PES framing at the source.
