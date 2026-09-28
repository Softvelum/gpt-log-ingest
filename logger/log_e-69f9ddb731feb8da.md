# Decoder metadata queue reached its limit

Product: Nimble Streamer
Component: Decoder metadata synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-69f9ddb731feb8da

## Log pattern

```text
max items count %d exceeded, clear queue for [%s]
```

## Parameters

- Argument 1 — Limit: Maximum retained metadata count, integer items.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
max items count 128 exceeded, clear queue for [live/camera]
```

## Short description

Decoder metadata queue reached its limit.

## Trigger condition

After backward-gap cleanup, the caption or timecode queue still holds at least 128 items before insertion.

## Explanation

The entire queue is cleared, then the new metadata item is inserted. Pending metadata is lost.

## Possible causes

- Metadata accumulated without matching decoded presentation timestamps draining it.

## What to check

1. Check decoder progress and correspondence between metadata and output frame timestamps.

## Recommended action

Resolve decoder stalls or timestamp mismatch to prevent repeated metadata loss.
