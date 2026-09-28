# DVR cleanup stream name is invalid

Product: Nimble Streamer
Component: DVR
Logging function: log_e
Log level: error
Documentation ID: log_e-dbf401efe4fd1eb2

## Log pattern

```text
invalid dvr archive stream='%s' to delete
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid dvr archive stream='live/camera' to delete
```

## Short description

DVR cleanup stream name is invalid.

## Trigger condition

The cleanup target does not split into exactly an application and stream.

## Explanation

Cleanup returns not found without modifying an archive.

## Possible causes

- The cleanup target does not split into exactly an application and stream.

## What to check

1. Supply a valid application/stream cleanup target.

## Recommended action

Supply a valid application/stream cleanup target.
