# Session begins waiting for live-stream changes

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-ba31b70e7a9f80b3

## Log pattern

```text
WorkerThread::AddStreamObserver stream='%s' session=%p
```

## Parameters

- Argument 1 — Stream name: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WorkerThread::AddStreamObserver stream='example-item' session=0x1000
```

## Short description

Session begins waiting for live-stream changes.

## Trigger condition

A session asks to observe an HTTP live stream.

## Explanation

The session is inserted into an existing set, or a new upstream subscription is created for MPEG-TS or Icecast. Duplicate or failed registration can still follow this message.
