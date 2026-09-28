# Transient input name already belongs to a live stream

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-cea68f44ec4b41ab

## Log pattern

```text
cannot add transient camera with id=%s. Live stream id is already in use
```

## Parameters

- Argument 1 — Identifier: Requested transient camera and stream identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot add transient camera with id=live/camera. Live stream id is already in use
```

## Short description

Transient input name already belongs to a live stream.

## Trigger condition

Creating a transient transport stream finds the requested identifier already in the live-stream registry.

## Explanation

The transient stream is not created. Camera registration continues despite this failure.

## Possible causes

- Creating a transient transport stream finds the requested identifier already in the live-stream registry.

## What to check

1. Check existing configured and transient streams with this identifier.

## Recommended action

Use a distinct transient application/stream name.
