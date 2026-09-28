# Transient input name already belongs to a camera

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-06ceafbe129b9495

## Log pattern

```text
cannot add transient camera with id=%s. Name is already in use
```

## Parameters

- Argument 1 — Identifier: Requested transient camera and stream identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot add transient camera with id=live/camera. Name is already in use
```

## Short description

Transient input name already belongs to a camera.

## Trigger condition

Creating a transient transport stream finds its identifier already present in the camera-to-stream mapping.

## Explanation

The transient stream is not created. The camera-registration caller ignores this failure and continues recording publisher and update state.

## Possible causes

- Creating a transient transport stream finds its identifier already present in the camera-to-stream mapping.

## What to check

1. Check whether another configured or transient input already uses this identifier.

## Recommended action

Choose a distinct transient application/stream name.
