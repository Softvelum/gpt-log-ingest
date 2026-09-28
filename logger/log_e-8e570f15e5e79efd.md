# Connection statistics lack application or stream

Product: Nimble Streamer
Component: Streaming statistics
Logging function: log_e
Log level: error
Documentation ID: log_e-8e570f15e5e79efd

## Log pattern

```text
Statistics::Connected invalid app='%s' stream='%s' shared=%d
```

## Parameters

- Argument 1 — Application: Application name as text.
- Argument 2 — Stream: Stream name as text.
- Argument 3 — Shared session: Integer flag: 1 for a shared session and 0 otherwise.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Statistics::Connected invalid app='' stream='camera' shared=0
```

## Short description

Connection statistics lack application or stream.

## Trigger condition

A connection notification has an empty application name or stream name.

## Explanation

No connection record is created and no counters are increased.

## Possible causes

- At least one required application or stream name is empty.

## What to check

1. Inspect both reported names.

## Recommended action

Provide nonempty application and stream names for the statistics notification.
