# Live-stream observer registration attempted

Product: Nimble Streamer
Component: HTTP live-stream notifications
Logging function: log_d
Log level: debug
Documentation ID: log_d-bfad6745e20e1e66

## Log pattern

```text
BaseHTTPLiveManager::AddObserver stream='%s' o=%p
```

## Parameters

- Argument 1 — Stream: Application/stream identifier.
- Argument 2 — Observer: Address identifying the notification consumer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseHTTPLiveManager::AddObserver stream='live/radio' o=0x12345000
```

## Short description

Live-stream observer registration attempted.

## Trigger condition

A consumer requests registration for notifications about a stream.

## Explanation

The observer is inserted into the stream notification set; a duplicate registration is not inserted again. Logging occurs before that result is known.

## Possible causes

- A consumer requests registration for notifications about a stream.

## What to check

1. Match the stream with the consumer awaiting chunks.
