# DASH media segment is cached

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_d
Log level: debug
Documentation ID: log_d-3a17bf77fe96ae06

## Log pattern

```text
add_dash_segment key='%s' duration=%llu,%llu size=%u
```

## Parameters

- Argument 1 — Cache key: Application/stream path and generated segment or part name used as the cache identity.
- Argument 2 — Whole seconds: Integral part of segment duration in seconds.
- Argument 3 — Milliseconds: Fractional duration remainder in milliseconds, from 0 through 999.
- Argument 4 — Size: Complete serialized segment size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
add_dash_segment key='/live/camera/v_100_0.m4s' duration=2,0 size=1048576
```

## Short description

DASH media segment is cached.

## Trigger condition

A media segment has been serialized, stored, and marked ready in the cache.

## Explanation

Its duration and estimated bandwidth are updated; the segment is then retained in the live segment list. This is a storage event rather than proof of client delivery.

## Possible causes

- A media segment has been serialized, stored, and marked ready in the cache.

## What to check

1. Check the segment key, duration, and size when diagnosing packaging.

## Recommended action

No action is required for expected segment production.
