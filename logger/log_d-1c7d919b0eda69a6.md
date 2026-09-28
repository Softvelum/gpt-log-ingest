# Scenario live stream destroyed

Product: Nimble Streamer
Component: Scenario encoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-1c7d919b0eda69a6

## Log pattern

```text
LiveStreamInfo::~LiveStreamInfo: %s/%s
```

## Parameters

- Argument 1 — Application: Live-stream application name.
- Argument 2 — Stream: Live-stream name within its application.

## Log examples

Illustrative examples; these are not captured server logs.

```text
LiveStreamInfo::~LiveStreamInfo: live/camera
```

## Short description

Scenario live stream destroyed.

## Trigger condition

The scenario encoder destroys its live-stream state.

## Explanation

Any special device-input media buffer owned by that stream is destroyed as part of cleanup. The message does not itself identify the reason the stream lifetime ended.

## Possible causes

- The scenario encoder destroys its live-stream state.

## What to check

1. Check earlier scenario messages if this cleanup was unexpected.

## Related log messages

- [Scenario live stream created](log_d-f77c761cfc0d3a6e.md)
