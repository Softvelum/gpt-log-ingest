# HLS chunk payload storage cannot be allocated

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-bb64cb08230aa4cb

## Log pattern

```text
add_chunk failed to allocate item key='%s' size=%d
```

## Parameters

- Argument 1 — Cache key: Application/stream path and generated segment or part name used as the cache identity.
- Argument 2 — Size: Requested cache payload size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
add_chunk failed to allocate item key='/live/camera/l_100_60000_10.ts' size=188000
```

## Short description

HLS chunk payload storage cannot be allocated.

## Trigger condition

The reserved chunk cache item cannot obtain storage for its payload.

## Explanation

The item is cancelled and released, and it is not added to the output list. Cache limits, zero size, memory allocation, or disk errors can cause this failure.

## Possible causes

- The reserved chunk cache item cannot obtain storage for its payload.

## What to check

1. Check the requested size, cache capacity, and lower-level storage errors.

## Recommended action

Restore storage capacity or correct the item size.
