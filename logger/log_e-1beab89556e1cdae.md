# No Quick Sync encoder surface is unlocked

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-1beab89556e1cdae

## Log pattern

```text
%p failed to get unlocked frame
```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to get unlocked frame
```

## Short description

No Quick Sync encoder surface is unlocked.

## Trigger condition

Every input surface remains locked when the encoder searches for storage for a new frame.

## Explanation

No frame is copied and the frame-writing operation fails.

## Possible causes

- Outstanding encoding work retains the entire configured surface pool.

## What to check

1. Check asynchronous encoding progress, preceding synchronization errors, and surface demand from lookahead.

## Recommended action

Resolve stalled device work and restart the encoder.
