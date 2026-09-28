# Invalid DVR database record was skipped

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_e
Log level: error
Documentation ID: log_e-64445ab4efb68f5c

## Log pattern

```text
skipping invalid record in file='%s' offset=%u
```

## Parameters

- Argument 1 — Segment database: Text naming the recording segment-index database.
- Argument 2 — Database offset: Unsigned byte offset of the segment record in its database.

## Log examples

Illustrative examples; these are not captured server logs.

```text
skipping invalid record in file='/data/dvr/live/camera/nimble-dvr-segments.db' offset=512
```

## Short description

Invalid DVR database record was skipped.

## Trigger condition

A database line fails the record-shape check or fallback parsing does not produce a JSON object.

## Explanation

The invalid record is skipped. The normal reader adds a newline when its final record is invalid to separate future appends.

## Possible causes

- A database line fails the record-shape check or fallback parsing does not produce a JSON object.

## What to check

1. Check the affected database record and whether its referenced recording is still needed.

## Recommended action

Check the affected database record and whether its referenced recording is still needed.
