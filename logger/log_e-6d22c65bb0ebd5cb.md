# HLS part cache reservation fails

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-6d22c65bb0ebd5cb

## Log pattern

```text
add_part failed to reserve item key='%s'
```

## Parameters

- Argument 1 — Cache key: Application/stream path and generated segment or part name used as the cache identity.

## Log examples

Illustrative examples; these are not captured server logs.

```text
add_part failed to reserve item key='/live/camera/l_100_60000_10_0.ts'
```

## Short description

HLS part cache reservation fails.

## Trigger condition

The generated part key cannot be reserved in the cache.

## Explanation

The item is not added to the live output list. An existing key or cache-item allocation failure can cause this result.

## Possible causes

- The generated part key cannot be reserved in the cache.

## What to check

1. Check duplicate keys and cache allocation errors.

## Recommended action

Resolve the cache collision or allocation failure.
