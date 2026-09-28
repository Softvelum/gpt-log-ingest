# Last live-stream waiting session removed

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-204e9faad2d5673a

## Log pattern

```text
WorkerThread::RemoveObserver stream='%s' removed empty set
```

## Parameters

- Argument 1 — Stream: Observed stream name as text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WorkerThread::RemoveObserver stream='live/camera' removed empty set
```

## Short description

Last live-stream waiting session removed.

## Trigger condition

The stream waiting-session set becomes empty.

## Explanation

Upstream MPEG-TS or Icecast observation is removed where relevant, and the empty set is erased.
