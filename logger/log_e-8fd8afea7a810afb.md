# H.265 configuration has no registered live stream

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-8fd8afea7a810afb

## Log pattern

```text
H265: unregistered stream
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
H265: unregistered stream
```

## Short description

H.265 configuration has no registered live stream.

## Trigger condition

The RTSP H.265 configuration is being applied while no live stream is attached.

## Explanation

Codec setup returns failure before registering the H.265 configuration. The message identifies missing stream registration rather than malformed media.

## Possible causes

- The RTSP H.265 configuration is being applied while no live stream is attached.

## What to check

1. Check whether the RTSP input has a registered live stream before codec setup.
