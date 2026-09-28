# DRM key consumers notified

Product: Nimble Streamer
Component: DRM key management
Logging function: log_d
Log level: debug
Documentation ID: log_d-eac5d86e3752dde2

## Log pattern

```text
notified key observers for '%s'
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
notified key observers for 'live/camera'
```

## Short description

DRM key consumers notified.

## Trigger condition

A stored listener set is removed and each listener receives the key operation result.

## Explanation

The listeners are no longer registered. This message does not distinguish success from failure.

## Possible causes

- A stored listener set is removed and each listener receives the key operation result.

## What to check

1. Check the preceding key result if the notification outcome is needed.

## Recommended action

Check the preceding key result if the notification outcome is needed.
