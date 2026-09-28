# DVR storage path is already attached

Product: Nimble Streamer
Component: DVR
Logging function: log_v
Log level: verbose
Documentation ID: log_v-b86bee88db1043e5

## Log pattern

```text
dvr archive stream='%s' is already loaded from path='%s'
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — Archive directory: Text giving the DVR storage directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
dvr archive stream='live/camera' is already loaded from path='/data/dvr/live/camera'
```

## Short description

DVR storage path is already attached.

## Trigger condition

The requested directory matches an existing DVR archive.

## Explanation

Duplicate loading is avoided where the existing archive still has settings; the message identifies that existing archive.

## Possible causes

- The requested directory matches an existing DVR archive.

## What to check

1. Check duplicate archive paths if another stream was meant to attach this storage.

## Recommended action

Check duplicate archive paths if another stream was meant to attach this storage.
