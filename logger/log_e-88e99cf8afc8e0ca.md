# Positive configuration value required

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-88e99cf8afc8e0ca

## Log pattern

```text
invalid parameter value: %s. should be > 0
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

The value is logged as its original text. Several settings share this pattern; identify the setting from the rejected configuration line.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0. should be > 0
```

## Short description

Positive configuration value required.

## Trigger condition

While reading configuration, a required positive value converts to zero or below.

## Explanation

Configuration processing reports failure and the file loader stops at this line.

## Possible causes

- A configured value violates the setting-specific range or accepted choices.

## What to check

1. Check the setting on the rejected configuration line; it requires a positive value for HTTP/3 worker/viewer counts; RTMP camera, publisher, worker and viewer counts; live-pull workers; DVR writer/management workers; WHEP workers/viewers; fragmented-MP4 HLS minimum/maximum delays in milliseconds and queue counts; Dante cache milliseconds; RTSP/WebSocket workers/viewers; UDP buffer bytes; RTSP frame-buffer bytes; or audio/video/x264 worker limits.

## Recommended action

Correct the value to a positive value for HTTP/3 worker/viewer counts; RTMP camera, publisher, worker and viewer counts; live-pull workers; DVR writer/management workers; WHEP workers/viewers; fragmented-MP4 HLS minimum/maximum delays in milliseconds and queue counts; Dante cache milliseconds; RTSP/WebSocket workers/viewers; UDP buffer bytes; RTSP frame-buffer bytes; or audio/video/x264 worker limits and reload configuration.
