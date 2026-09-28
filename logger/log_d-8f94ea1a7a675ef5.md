# Transport restreaming retention stops

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-8f94ea1a7a675ef5

## Log pattern

```text
restreaming stopped for cam=%s
```

## Parameters

- Argument 1 — Camera ID: Transport input camera identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
restreaming stopped for cam=camera1
```

## Short description

Transport restreaming retention stops.

## Trigger condition

A stop request reaches the camera restreaming buffer.

## Explanation

The item ring and partial packet item are cleared and the write index resets. Subsequent packets are ignored successfully until retention starts again.

## Possible causes

- A stop request reaches the camera restreaming buffer.

## What to check

1. Check whether all restreaming consumers were removed.

## Recommended action

No action is needed for an intentional stop.
