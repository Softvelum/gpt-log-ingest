# Unregistering a known live stream

Product: Nimble Streamer
Component: Live stream registration and transcription
Logging function: log_v
Log level: verbose
Documentation ID: log_v-d81bc314c8ce4ac0

## Log pattern

```text
LiveStreamManager::unregisterStream name='%s'
```

## Parameters

- Argument 1 — stream: Application and stream name joined by a slash.

## Log examples

Illustrative examples; these are not captured server logs.

```text
LiveStreamManager::unregisterStream name='live/camera'
```

## Short description

Unregistering a known live stream.

## Trigger condition

The stream object being removed is present in the reverse registration table.

## Explanation

The stream is marked dead and removed from name and identity tables. The main server also removes HTTP live delivery registration and sends an unpublish notification when enabled; the scenario encoder instead queues the name for notification to its parent process.
