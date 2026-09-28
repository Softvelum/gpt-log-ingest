# DVR HEVC blackout codec settings are invalid

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_e
Log level: error
Documentation ID: log_e-60e37775efc7b0ef

## Log pattern

```text
failed to parse hevc codec param, codec=%s
```

## Parameters

- Argument 1 — HEVC codec: Text containing the recorded HEVC codec string.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse hevc codec param, codec=invalid
```

## Short description

DVR HEVC blackout codec settings are invalid.

## Trigger condition

The recorded HEVC codec string cannot supply profile, tier, and level for a black replacement segment.

## Explanation

Blackout of the affected segment returns failure.

## Possible causes

- The recorded HEVC codec string cannot supply profile, tier, and level for a black replacement segment.

## What to check

1. Check the recorded HEVC initialization properties.

## Recommended action

Check the recorded HEVC initialization properties.
