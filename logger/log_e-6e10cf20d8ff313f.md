# SCTE-35 marker start is invalid

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-6e10cf20d8ff313f

## Log pattern

```text
invalid scte35 marker Start='%s'
```

## Parameters

- Argument 1 — Marker start: Text containing the rejected start value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid scte35 marker Start='invalid-date'
```

## Short description

SCTE-35 marker start is invalid.

## Trigger condition

A marker start value is neither digits nor a successfully parsed UTC timestamp.

## Explanation

The marker is excluded from the schedule.

## Possible causes

- A marker start value is neither digits nor a successfully parsed UTC timestamp.

## What to check

1. Use a numeric offset or a UTC date and time in YYYY-MM-DD HH:MM:SS form.

## Recommended action

Use a numeric offset or a UTC date and time in YYYY-MM-DD HH:MM:SS form.
