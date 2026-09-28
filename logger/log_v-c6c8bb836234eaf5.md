# Outdated decoder metadata removed

Product: Nimble Streamer
Component: Decoder metadata synchronization
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c6c8bb836234eaf5

## Log pattern

```text
forward gap detected, removed item with dts=%lld pts=%lld for [%s] pts=%lld
```

## Parameters

- Argument 1 — Removed decode timestamp: Decode timestamp of the removed metadata item, signed 64-bit ticks in the associated video time base.
- Argument 2 — Queued presentation timestamp: Presentation timestamp of the queued metadata item, signed 64-bit ticks in the associated video time base.
- Argument 3 — Stream: Input stream name, text.
- Argument 4 — Requested presentation timestamp: Decoded frame timestamp requested for matching, unsigned 64-bit ticks printed in decimal in the same video time base.

The timestamp arguments use fixed-width 64-bit types without casts to the signed long-long conversions. On platforms where those types use long, the printf types differ; incoming unsigned timestamps also require a representable signed value.

## Log examples

The timestamp arguments use fixed-width 64-bit types without casts to the signed long-long conversions. On platforms where those types use long, the printf types differ; incoming unsigned timestamps also require a representable signed value. A reliable example is omitted because the original call does not ensure matching printf argument types.

## Short description

Outdated decoder metadata removed.

## Trigger condition

A decoded frame requests metadata at a presentation timestamp later than a queued item.

## Explanation

Each older item is removed; only an exact presentation-time match is returned for the frame.
