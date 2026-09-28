# DASH or fragmented MP4 HLS cache data cannot be allocated

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-34814286fabbe7bc

## Log pattern

```text
failed to allocate item key='%s' size=%d
```

## Parameters

- Argument 1 — Cache key: Application/stream path and generated segment or part name used as the cache identity.
- Argument 2 — Size: Number of bytes requested for the cached item, including encryption output when applicable.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to allocate item key='/live/camera/v_100_0.m4s' size=1048576
```

## Short description

DASH or fragmented MP4 HLS cache data cannot be allocated.

## Trigger condition

A reserved DASH media/subtitle segment or fragmented MP4 HLS segment/part cannot obtain storage for its bytes.

## Explanation

The reserved item is cancelled and released. DASH media also finalizes timing and resets its builder. Cache limits, allocation or disk errors, and zero-sized data can cause this failure.

## Possible causes

- A reserved DASH media/subtitle segment or fragmented MP4 HLS segment/part cannot obtain storage for its bytes.

## What to check

1. Check the byte count, cache capacity, and corresponding allocation or disk errors.

## Recommended action

Restore cache capacity or correct the invalid item size.
