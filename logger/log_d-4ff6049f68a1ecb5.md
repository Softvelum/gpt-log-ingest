# RTSP publisher requires reconnect for newly available audio

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_d
Log level: debug
Documentation ID: log_d-4ff6049f68a1ecb5

## Log pattern

```text
failed to add audio on fly, force reconnect, client: %d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to add audio on fly, force reconnect, client: 25
```

## Short description

RTSP publisher requires reconnect for newly available audio.

## Trigger condition

The live buffer contains audio while the corresponding RTP data or control channel has not been set up.

## Explanation

Publishing returns failure so the existing connection can be removed and recreated with a fresh media announcement. Tracks cannot be added through this active-session path.

## Possible causes

- The live buffer contains audio while the corresponding RTP data or control channel has not been set up.

## What to check

1. Check whether audio became available after the initial publishing negotiation.

## Recommended action

Allow the publisher to reconnect and announce the available tracks.

## Related log messages

- [RTSP publisher requires reconnect for newly available video](log_d-a024ee09a3d5fb31.md)
