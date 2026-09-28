# DVR database cannot be opened for deletion

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_e
Log level: error
Documentation ID: log_e-e3b6ae32a1e8f091

## Log pattern

```text
failed to open segment db='%s' error='%s'
```

## Parameters

- Argument 1 — Segment database: Text naming the recording segment-index database.
- Argument 2 — Error message: Text describing the reported filesystem failure.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open segment db='/data/dvr/live/camera/nimble-dvr-segments.db' error='Permission denied'
```

## Short description

DVR database cannot be opened for deletion.

## Trigger condition

The cached read/write stream cannot open the segment database.

## Explanation

The target record is not marked deleted.

## Possible causes

- The cached read/write stream cannot open the segment database.

## What to check

1. Check database permissions and the reported filesystem error.

## Recommended action

Check database permissions and the reported filesystem error.
