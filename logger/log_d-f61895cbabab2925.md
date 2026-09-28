# PES payload exceeds its declared size

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-f61895cbabab2925

## Log pattern

```text
wrong PES packet size (%d) pid=%d
```

## Parameters

- Argument 1 — Payload length: Expected PES payload bytes after subtracting parsed header overhead, not the original raw PES length field.
- Argument 2 — PID: Affected elementary-stream packet identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong PES packet size (100) pid=256
```

## Short description

PES payload exceeds its declared size.

## Trigger condition

Appending the current transport payload would exceed the positive remaining PES payload length.

## Explanation

The incoming payload is rejected. The stream parser is reset, buffered partial PES data is discarded, and transport input searches for a new packet boundary.

## Possible causes

- Appending the current transport payload would exceed the positive remaining PES payload length.

## What to check

1. Compare declared PES length, header overhead, payload bytes, and transport stuffing.

## Recommended action

Correct the source PES length or packet padding.
