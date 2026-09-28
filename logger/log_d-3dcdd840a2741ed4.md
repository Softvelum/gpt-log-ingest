# DVR time-bucket directory removed

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_d
Log level: debug
Documentation ID: log_d-3dcdd840a2741ed4

## Log pattern

```text
dvr archive dir='%s' removed
```

## Parameters

- Argument 1 — Archive directory: Text giving the archive or time-bucket directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
dvr archive dir='/data/dvr/live/camera/1700000000000' removed
```

## Short description

DVR time-bucket directory removed.

## Trigger condition

Directory removal reports that one or more entries were removed after the contents check.

## Explanation

The directory cleanup succeeds.

## Possible causes

- Directory removal reports that one or more entries were removed after the contents check.
