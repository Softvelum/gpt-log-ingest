# DVR recording context is being removed

Product: Nimble Streamer
Component: DVR recording
Logging function: log_d
Log level: debug
Documentation ID: log_d-1fc1bdda0bc16c5b

## Log pattern

```text
deleting dvr context for stream='%s' path='%s'
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — Archive directory: Text giving the recording directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
deleting dvr context for stream='live/camera' path='/data/dvr/live/camera'
```

## Short description

DVR recording context is being removed.

## Trigger condition

The recording context destructor starts.

## Explanation

It first attempts to process remaining buffered media and save the last audio/video segments.

## Possible causes

- The recording context destructor starts.

## What to check

1. Check the preceding settings or stream change if removal was unexpected.

## Recommended action

Check the preceding settings or stream change if removal was unexpected.
