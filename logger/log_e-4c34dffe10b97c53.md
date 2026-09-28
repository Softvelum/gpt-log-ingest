# RTSP playback requested before track setup

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-4c34dffe10b97c53

## Log pattern

```text
Stream Not Setup, client=%d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Stream Not Setup, client=25
```

## Short description

RTSP playback requested before track setup.

## Trigger condition

PLAY uses the correct base URI but neither an audio nor video data channel has been set up.

## Explanation

The playback connection requests shutdown before sending a successful PLAY response.

## Possible causes

- PLAY uses the correct base URI but neither an audio nor video data channel has been set up.

## What to check

1. Check that a successful SETUP preceded PLAY.

## Recommended action

Set up at least one media track before requesting playback.
