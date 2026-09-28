# Completed origin response left an incomplete cache item

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-bab1e8f29925ab71

## Log pattern

```text
failed to download cache item key=%s size=%llu capacity=%llu
```

## Parameters

- Argument 1 — Cache key: Text identifying the requested media or playlist cache item.
- Argument 2 — Cached bytes: Unsigned 64-bit bytes stored in the item.
- Argument 3 — Expected capacity: Unsigned 64-bit allocated cache item size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to download cache item key=/live/camera/segment1.ts size=32768 capacity=65536
```

## Short description

Completed origin response left an incomplete cache item.

## Trigger condition

HTTP parsing reports a complete media body, but the reserved cache item size differs from its allocated capacity.

## Explanation

The cache item is canceled and released and the client session is marked finished.

## Possible causes

- HTTP parsing reports a complete media body, but the reserved cache item size differs from its allocated capacity.

## What to check

1. Compare the origin Content-Length with received bytes and check origin truncation or inconsistent response framing.

## Recommended action

Compare the origin Content-Length with received bytes and check origin truncation or inconsistent response framing.
