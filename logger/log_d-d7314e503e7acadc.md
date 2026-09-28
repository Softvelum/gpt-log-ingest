# DVR segment database is being loaded

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_d
Log level: debug
Documentation ID: log_d-d7314e503e7acadc

## Log pattern

```text
loading dvr segments db='%s'
```

## Parameters

- Argument 1 — Segment database: Text naming the recording segment-index database.

## Log examples

Illustrative examples; these are not captured server logs.

```text
loading dvr segments db='/data/dvr/live/camera/nimble-dvr-segments.db'
```

## Short description

DVR segment database is being loaded.

## Trigger condition

A numeric time-bucket directory contains a segment database.

## Explanation

The database is parsed and its recorded entries are examined.

## Possible causes

- A numeric time-bucket directory contains a segment database.
