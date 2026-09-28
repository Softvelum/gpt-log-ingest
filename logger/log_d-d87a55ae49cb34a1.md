# Video filter worker stopped

Product: Nimble Streamer
Component: Video filtering
Logging function: log_d
Log level: debug
Documentation ID: log_d-d87a55ae49cb34a1

## Log pattern

```text
VideoFilterThread(%s) stopped
```

## Parameters

- Argument 1 — Filter: Configured filter identifier, text.

One occurrence guards initial filter creation, whose current helper returns success unconditionally. The reachable loop-exit occurrence also covers processing failures.

## Log examples

Illustrative examples; these are not captured server logs.

```text
VideoFilterThread(overlay1) stopped
```

## Short description

Video filter worker stopped.

## Trigger condition

The filter worker leaves its processing loop after a stop request or filtering failure, or takes its initial filter-creation failure guard.

## Explanation

The worker returns and no further frames are processed by this instance.
