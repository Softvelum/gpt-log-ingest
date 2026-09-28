# Duplicate metadata presentation timestamp detected

Product: Nimble Streamer
Component: Decoder metadata synchronization
Logging function: log_i
Log level: info
Documentation ID: log_i-2b3b60b6ceabe32b

## Log pattern

```text
multiple items with same pts=%lld detected for [%s]
```

## Parameters

- Argument 1 — Queued presentation timestamp: Presentation timestamp of the queued metadata item, signed 64-bit ticks in the associated video time base.
- Argument 2 — Stream: Input stream name, text.

The timestamp argument uses a fixed-width signed 64-bit type without a cast to the signed long-long conversion. On platforms where that typedef uses signed long, the printf types differ.

## Log examples

The timestamp argument uses a fixed-width signed 64-bit type without a cast to the signed long-long conversion. On platforms where that typedef uses signed long, the printf types differ. A reliable example is omitted because the original call does not ensure matching printf argument types.

## Short description

Duplicate metadata presentation timestamp detected.

## Trigger condition

An item being inserted has the same presentation timestamp as an existing caption or timecode item.

## Explanation

The duplicate is reported and the new item is still inserted; this check does not replace or discard either item.
