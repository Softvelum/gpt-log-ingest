# Stream removal observers notified

Product: Nimble Streamer
Component: HTTP live-stream notifications
Logging function: log_d
Log level: debug
Documentation ID: log_d-b07fb4dcb5c57fa7

## Log pattern

```text
BaseHTTPLiveManager::NotifyStreamUnregistered stream='%s'
```

## Parameters

- Argument 1 — Stream: Application/stream identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseHTTPLiveManager::NotifyStreamUnregistered stream='live/radio'
```

## Short description

Stream removal observers notified.

## Trigger condition

A stream-unregistered notification is issued.

## Explanation

Registered observers are told that the stream has been removed, allowing them to react to stream loss.

## Possible causes

- A stream-unregistered notification is issued.

## What to check

1. Check whether the corresponding source stream ended.
