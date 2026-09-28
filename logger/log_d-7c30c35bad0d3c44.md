# Live-stream removal queued

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-7c30c35bad0d3c44

## Log pattern

```text
WorkerThread::StreamUnregistered stream='%s'
```

## Parameters

- Argument 1 — Stream: Observed stream name as text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WorkerThread::StreamUnregistered stream='live/camera'
```

## Short description

Live-stream removal queued.

## Trigger condition

The worker receives an upstream stream-unregistered notification.

## Explanation

The stream is added to both the changed-stream set and deletion set for later dispatch.
