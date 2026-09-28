# Session notification handling raised an exception

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_e
Log level: error
Documentation ID: log_e-60fab626b84d41ab

## Log pattern

```text
unknown exception while processing session=%p
```

## Parameters

- Argument 1 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception while processing session=0x1000
```

## Short description

Session notification handling raised an exception.

## Trigger condition

A session callback throws while handling cache updates, live-stream changes, advertising events, segmented-stream polling or timer delivery.

## Explanation

The affected session is marked finished and deleted. Other queued notifications or timers continue to be processed.

## Possible causes

- A notification or timer callback raised an unclassified exception.

## What to check

1. Use the preceding notification message to identify the event and session.
