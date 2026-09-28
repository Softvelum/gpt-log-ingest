# Cache fragment index is outside the item

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-928a67ddfdbbd673

## Log pattern

```text
invalid fragment index=%llu for item key='%s' fragments_count=%lu
```

## Parameters

- Argument 1 — fragment: Zero-based fragment index.
- Argument 2 — key: Cache lookup key identifying the stored item.
- Argument 3 — fragment count: Number of fragments represented by the item.

The fragment count is a 32-bit unsigned value passed to an unsigned-long conversion; printed output depends on the build platform.

## Log examples

No portable example is provided because the fragment-count argument type differs from the conversion.

## Short description

Cache fragment index is outside the item.

## Trigger condition

The requested fragment index is at least the number of fragments in the item.

## Explanation

Fragment allocation returns failure before creating a file.

## Possible causes

- A fragment outside the item range was requested.

## What to check

1. Compare the requested index with the item capacity and fragment size.

## Recommended action

Investigate the invalid fragment request.
