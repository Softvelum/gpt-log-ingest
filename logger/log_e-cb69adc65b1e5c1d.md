# WHIP helper returned an unknown session

Product: Nimble Streamer
Component: WHIP publishing helper
Logging function: log_e
Log level: error
Documentation ID: log_e-cb69adc65b1e5c1d

## Log pattern

```text
cannot find whip session with id=%d
```

## Parameters

- Argument 1 — Session identifier: WebRTC helper session identifier, unsigned 32-bit integer displayed in decimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot find whip session with id=7
```

## Short description

WHIP helper returned an unknown session.

## Trigger condition

A helper connection identifies itself as a WHIP media session whose identifier is absent from the pending requested-session map.

## Explanation

The new session socket is removed from monitoring and closed.

## Possible causes

- The requested session was removed, expired, or belongs to an earlier helper connection.

## What to check

1. Compare this identifier with recent session-start and helper-reconnection events.

## Recommended action

Retry the publication; investigate helper/session timing if the rejection repeats.
