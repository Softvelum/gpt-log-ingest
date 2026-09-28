# WHEP media worker stopped

Product: Nimble Streamer
Component: WHEP media delivery
Logging function: log_d
Log level: debug
Documentation ID: log_d-eac995bf6684a61f

## Log pattern

```text
WhepWebRTCWorkerThread stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
WhepWebRTCWorkerThread stopped
```

## Short description

WHEP media worker stopped.

## Trigger condition

The media worker handles an exit request and leaves its run loop.

## Explanation

All sessions owned by the worker have been removed from active playback, marked aborted, and cleared.
