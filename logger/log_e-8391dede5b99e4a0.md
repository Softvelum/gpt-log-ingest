# RTSP media type is unsupported

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-8391dede5b99e4a0

## Log pattern

```text
unsupported codec type='%s' stream url='%s'
```

## Parameters

- Argument 1 — Media type: SDP media category string.
- Argument 2 — Stream URL: RTSP input base URL identifying the affected source.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported codec type='text' stream url='rtsp://192.0.2.1/live'
```

## Short description

RTSP media type is unsupported.

## Trigger condition

The SDP media type is neither video, audio, nor application.

## Explanation

The track cannot be assigned a supported RTSP stream type and codec selection returns failure.

## Possible causes

- The SDP media type is neither video, audio, nor application.

## What to check

1. Check the media type advertised in SDP.

## Recommended action

Use a supported media type for the RTSP track.
