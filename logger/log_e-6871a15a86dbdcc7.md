# WHEP helper returned an unknown session

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-6871a15a86dbdcc7

## Log pattern

```text
cannot find whep session with id=%d
```

## Parameters

- Argument 1 — Session identifier: WebRTC helper session identifier, unsigned 32-bit integer displayed in decimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot find whep session with id=7
```

## Short description

WHEP helper returned an unknown session.

## Trigger condition

A returned WHEP media connection names an identifier absent from the requested-session map.

## Explanation

The returned media socket is removed from monitoring and closed instead of being handed to a playback worker.

## Possible causes

- The request expired or was cleared, or the helper returns a stale session identifier.

## What to check

1. Compare this identifier with earlier session requests and control reconnections.

## Recommended action

Retry playback; investigate repeated stale helper replies.
