# Disk cache protocol is unsupported

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-c252eb13cf4ec13e

## Log pattern

```text
DataCache::AllocateItemData invalid protocol=%d
```

## Parameters

- Argument 1 — protocol: Numeric session protocol supplied to cache allocation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DataCache::AllocateItemData invalid protocol=0
```

## Short description

Disk cache protocol is unsupported.

## Trigger condition

Disk allocation is selected for a session protocol without a cache-directory mapping.

## Explanation

The cache returns allocation failure before creating the item file.

## Possible causes

- The supplied protocol is not supported by this disk-cache allocation path.

## What to check

1. Identify the request protocol and the preceding allocation decision.

## Recommended action

Investigate why this protocol reached disk cache allocation.
