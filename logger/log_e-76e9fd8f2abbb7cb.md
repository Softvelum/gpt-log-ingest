# Cache or stream waiting set was not found

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_e
Log level: error
Documentation ID: log_e-76e9fd8f2abbb7cb

## Log pattern

```text
WorkerThread::RemoveObserver key='%s' session=%p not found
```

## Parameters

- Argument 1 — Resource identifier: Text identifying the cache item or live stream.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WorkerThread::RemoveObserver key='example-item' session=0x1000 not found
```

## Short description

Cache or stream waiting set was not found.

## Trigger condition

Removing a notification subscription finds no set for the cache item or live stream.

## Explanation

No waiting set is changed. The shared cache/stream pattern identifies either a cache key or a stream name according to the operation.

## Possible causes

- The requested resource has no registered waiting-session set.

## What to check

1. Compare subscription registration and removal for the same resource.
