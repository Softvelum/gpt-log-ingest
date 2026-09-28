# DVR deletion record cannot be reached

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_e
Log level: error
Documentation ID: log_e-21ce38ebdc6aec4c

## Log pattern

```text
failed to seek to pos=%u in segment db='%s'
```

## Parameters

- Argument 1 — Database offset: Unsigned byte offset of the segment record in its database.
- Argument 2 — Segment database: Text naming the recording segment-index database.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to seek to pos=512 in segment db='/data/dvr/live/camera/nimble-dvr-segments.db'
```

## Short description

DVR deletion record cannot be reached.

## Trigger condition

The database stream cannot seek to the target record offset.

## Explanation

The record is not marked deleted.

## Possible causes

- The database stream cannot seek to the target record offset.

## What to check

1. Check database integrity and the reported record offset.

## Recommended action

Check database integrity and the reported record offset.
