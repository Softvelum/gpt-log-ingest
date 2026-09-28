# PES fixed header does not fit its first packet

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-7cc10fe6f67ebe1a

## Log pattern

```text
PES header MUST be in PUSI PES block pid=%d
```

## Parameters

- Argument 1 — PID: Affected elementary-stream packet identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
PES header MUST be in PUSI PES block pid=256
```

## Short description

PES fixed header does not fit its first packet.

## Trigger condition

A payload-start transport packet has fewer than nine bytes remaining for the fixed PES header.

## Explanation

The current transport packet is rejected. Its elementary-stream parser is reset, buffered partial PES data is discarded, and the camera parser searches for another packet boundary.

## Possible causes

- A payload-start transport packet has fewer than nine bytes remaining for the fixed PES header.

## What to check

1. Check the adaptation-field length and placement of the PES header.

## Recommended action

Correct malformed PES framing at the source.
