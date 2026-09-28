# DVR database append position is unavailable

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_e
Log level: error
Documentation ID: log_e-e2340dce19e8feed

## Log pattern

```text
failed to seek to the end of segment db='%s'
```

## Parameters

- Argument 1 — Segment database: Text naming the recording segment-index database.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to seek to the end of segment db='/data/dvr/live/camera/nimble-dvr-segments.db'
```

## Short description

DVR database append position is unavailable.

## Trigger condition

Version-1 recording cannot seek to the end of its segment database.

## Explanation

The pending segment record is not written.

## Possible causes

- Version-1 recording cannot seek to the end of its segment database.

## What to check

1. Check database file integrity and filesystem I/O errors.

## Recommended action

Check database file integrity and filesystem I/O errors.
