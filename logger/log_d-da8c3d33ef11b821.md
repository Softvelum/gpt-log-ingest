# Owned incomplete cache item is canceled during cleanup

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-da8c3d33ef11b821

## Log pattern

```text
canceled cache item key='%s' state=%d size=%llu capacity=%llu
```

## Parameters

- Argument 1 — Cache key: Text identifying the requested media or playlist cache item.
- Argument 2 — Cache state: Integer cache state: 0 newly reserved, 1 receiving data, 2 fragmented, 3 ready, or 4 marked for removal.
- Argument 3 — Stored bytes: Unsigned 64-bit number of bytes stored in the cache item.
- Argument 4 — Capacity bytes: Unsigned 64-bit allocated or logical total capacity in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
canceled cache item key='/live/camera/segment1.ts' state=4 size=32768 capacity=65536
```

## Short description

Owned incomplete cache item is canceled during cleanup.

## Trigger condition

A request releases an item it reserved, and the capacity is zero or the stored size is below capacity.

## Explanation

Cleanup attempts to cancel the incomplete item and then releases its references. HTTP/2 and HTTP/3 cleanup can also emit this after catching an exception from cancellation, so the displayed state matters.

## Possible causes

- A request releases an item it reserved, and the capacity is zero or the stored size is below capacity.

## What to check

1. Check why the owning request ended before its download completed, especially preceding client closure, origin failure, or cache errors.

## Recommended action

Check why the owning request ended before its download completed, especially preceding client closure, origin failure, or cache errors.
