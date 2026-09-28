# Scenario live stream created

Product: Nimble Streamer
Component: Scenario encoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-f77c761cfc0d3a6e

## Log pattern

```text
LiveStreamInfo::LiveStreamInfo: %s/%s
```

## Parameters

- Argument 1 — Application: Live-stream application name.
- Argument 2 — Stream: Live-stream name within its application.

## Log examples

Illustrative examples; these are not captured server logs.

```text
LiveStreamInfo::LiveStreamInfo: live/camera
```

## Short description

Scenario live stream created.

## Trigger condition

The scenario encoder creates its live-stream state for an application/stream pair.

## Explanation

For NDI, OMT, and SDI applications, a corresponding media buffer is also created to trigger device-input handling. Other applications retain stream state without that special buffer creation.

## Possible causes

- Normal creation of scenario live-stream state.

## Related log messages

- [Scenario live stream destroyed](log_d-1c7d919b0eda69a6.md)
