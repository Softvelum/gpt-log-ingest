# RTSP application encoding is unsupported

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-fc5b9e78d2ff2eb1

## Log pattern

```text
unsupported application codec='%s' stream url='%s'
```

## Parameters

- Argument 1 — Encoding: SDP application encoding name.
- Argument 2 — Stream URL: RTSP input base URL identifying the affected source.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported application codec='unsupported-data' stream url='rtsp://192.0.2.1/live'
```

## Short description

RTSP application encoding is unsupported.

## Trigger condition

An SDP application track uses an encoding other than vnd.onvif.metadata.

## Explanation

RTSP codec selection fails. Recognized ONVIF metadata tracks are accepted and their media data is skipped.

## Possible causes

- An SDP application track uses an encoding other than vnd.onvif.metadata.

## What to check

1. Check the application encoding in SDP.

## Recommended action

Use the supported ONVIF metadata encoding or remove the unsupported application track.
