# Live chunk observers notified

Product: Nimble Streamer
Component: HTTP live-stream notifications
Logging function: log_d
Log level: debug
Documentation ID: log_d-d7a8dcdf1c28eafa

## Log pattern

```text
BaseHTTPLiveManager::NotifyStreamChunkAdded stream='%s'
```

## Parameters

- Argument 1 — Stream: Application/stream identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseHTTPLiveManager::NotifyStreamChunkAdded stream='live/radio'
```

## Short description

Live chunk observers notified.

## Trigger condition

A chunk-added notification is issued for a stream.

## Explanation

Every currently registered observer is called. The log can occur even when no observer is registered.

## Possible causes

- A chunk-added notification is issued for a stream.

## What to check

1. Check that waiting consumers are registered for the same stream.

## Related log messages

- [Live-stream observer registration attempted](log_d-bfad6745e20e1e66.md)
