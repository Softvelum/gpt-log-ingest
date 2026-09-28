# SCTE-35 marker type is invalid

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-1b9bdf366b39bea0

## Log pattern

```text
invalid scte35 marker Type='%s'
```

## Parameters

- Argument 1 — Marker type: Text containing the rejected marker type.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid scte35 marker Type='Unknown'
```

## Short description

SCTE-35 marker type is invalid.

## Trigger condition

A marker type is neither In nor Out.

## Explanation

The marker is excluded from the schedule.

## Possible causes

- A marker type is neither In nor Out.

## What to check

1. Set the marker type to In or Out.

## Recommended action

Set the marker type to In or Out.
