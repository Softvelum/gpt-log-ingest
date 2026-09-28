# VCAS key set is shorter than configured

Product: Nimble Streamer
Component: DRM key management
Logging function: log_d
Log level: debug
Documentation ID: log_d-1e7eabc16b66b1bf

## Log pattern

```text
keys.size()=%llu < pos_count=%u
```

## Parameters

- Argument 1 — Stored keys: Unsigned count of stored key entries.
- Argument 2 — Configured keys: Unsigned required key count.

## Log examples

Illustrative examples; these are not captured server logs.

```text
keys.size()=1 < pos_count=3
```

## Short description

VCAS key set is shorter than configured.

## Trigger condition

The stored VCAS key set contains fewer entries than the configured rotating-key count.

## Explanation

The HLS operation fails unless fallback to the configured default key is permitted; with fallback permitted, default-key selection is attempted.

## Possible causes

- The stored VCAS key set contains fewer entries than the configured rotating-key count.

## What to check

1. Check the key-count setting and whether all required keys were registered.

## Recommended action

Check the key-count setting and whether all required keys were registered.
