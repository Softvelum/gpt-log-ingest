# HLS advertising skips source chunk indices

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-d9d4a5d0f47a08cf

## Log pattern

```text
app=%s stream=%s stream_name=%s session=%lu: missed %d chunks, sequence=%d
```

## Parameters

- Argument 1 — Application: Application associated with the advertising playback session.
- Argument 2 — Stream: Stream associated with the advertising playback session.
- Argument 3 — Rendition: Specific stream name being processed for this session.
- Argument 4 — Session: Numeric playback session identifier.
- Argument 5 — Index delta: Difference between current and previously processed chunk indices; this 64-bit value is formatted with a 32-bit conversion.
- Argument 6 — Sequence: Session sequence before the adjustment; this 64-bit value is formatted with a 32-bit conversion.

## Log examples

No reliable complete example is available because the chunk delta and sequence are 64-bit values passed to 32-bit conversions.

## Short description

HLS advertising skips source chunk indices.

## Trigger condition

A newly available source chunk index advances by more than one from the last processed chunk.

## Explanation

The session sequence is advanced by that index delta and a discontinuity is assumed. The printed missed count is the full index delta, not the delta minus one; both numeric displays have width mismatches.

## Possible causes

- A newly available source chunk index advances by more than one from the last processed chunk.

## What to check

1. Compare actual source and session playlist sequences and check delayed advertising processing or client requests.

## Recommended action

Resolve repeated processing delays that let source chunks leave the retained window.
