# Stream publishing starts

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_v
Log level: verbose
Documentation ID: log_v-9bdfd0db821d1dfe

## Log pattern

```text
start publishing %s
```

## Parameters

- Argument 1 — Publishing route: Human-readable source application/stream and destination URL; RTMP and RTSP also include the connected peer IP.

## Log examples

Illustrative examples; these are not captured server logs.

```text
start publishing stream='live/camera' to 'rtmp://destination.example:1935/live/camera' (192.0.2.20)
```

## Short description

Stream publishing starts.

## Trigger condition

RTMP accepts the publish response, RTSP accepts RECORD, or the HLS publisher prepares its first nonempty update for sending.

## Explanation

The message identifies the source and destination route. RTMP and RTSP emit it when a peer address is available. HLS emits it before sending its first playlist request, so that occurrence does not confirm a successful upload.

## Possible causes

- RTMP accepts the publish response, RTSP accepts RECORD, or the HLS publisher prepares its first nonempty update for sending.

## What to check

1. Check the named route and subsequent publish or upload errors.

## Recommended action

No action is required when this route is expected.
