# DVR shared-storage directory created

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_d
Log level: debug
Documentation ID: log_d-2d3e50aee47f27b4

## Log pattern

```text
dvr archive directory='%s' created
```

## Parameters

- Argument 1 — Archive directory: Text giving the archive or time-bucket directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
dvr archive directory='/data/dvr/live/camera/1700000000000' created
```

## Short description

DVR shared-storage directory created.

## Trigger condition

Version-2 recording needs a missing time-bucket directory and directory creation succeeds.

## Explanation

Recording continues with the new directory.

## Possible causes

- Version-2 recording needs a missing time-bucket directory and directory creation succeeds.
