# Live-stream media update queued

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-ebbf0422167ffa29

## Log pattern

```text
WorkerThread::StreamChunkAdded stream='%s'
```

## Parameters

- Argument 1 — Stream: Observed stream name as text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WorkerThread::StreamChunkAdded stream='live/camera'
```

## Short description

Live-stream media update queued.

## Trigger condition

The worker receives a notification that a stream chunk was added.

## Explanation

The stream is added to the protocol-specific changed set for later delivery to waiting sessions.
