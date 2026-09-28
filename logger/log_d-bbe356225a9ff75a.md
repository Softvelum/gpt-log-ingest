# WHEP control worker started

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_d
Log level: debug
Documentation ID: log_d-bbe356225a9ff75a

## Log pattern

```text
WhepWebRTCControlThread started
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
WhepWebRTCControlThread started
```

## Short description

WHEP control worker started.

## Trigger condition

The WHEP control worker begins running.

## Explanation

The helper listener is registered, then the worker waits briefly for transcoder availability before managing playback session starts.
