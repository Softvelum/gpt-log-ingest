# Unrefreshed transport elementary stream is removed

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-b605f9f554cdfb7a

## Log pattern

```text
orphan stream discarded(pid=%d,type=%d)
```

## Parameters

- Argument 1 — PID: Removed elementary-stream packet identifier.
- Argument 2 — Type: Last recorded stream type code in decimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
orphan stream discarded(pid=256,type=27)
```

## Short description

Unrefreshed transport elementary stream is removed.

## Trigger condition

An elementary stream is still marked unrefreshed when its program map undergoes the next orphan check.

## Explanation

That PID is removed from the available-stream map and the map is marked changed.

## Possible causes

- An elementary stream is still marked unrefreshed when its program map undergoes the next orphan check.

## What to check

1. Check whether recent PMT sections still list this PID.

## Recommended action

Correct missing/incomplete program-map updates if the stream should remain available.
