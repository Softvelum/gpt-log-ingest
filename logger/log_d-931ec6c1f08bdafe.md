# Saved cache fragment file is missing

Product: Nimble Streamer
Component: Media data cache
Logging function: log_d
Log level: debug
Documentation ID: log_d-931ec6c1f08bdafe

## Log pattern

```text
missing fragment key='%s' fi=%u
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — fragment: Zero-based fragment index.

The fragment index is a platform-sized value passed to a 32-bit unsigned conversion.

## Log examples

No portable example is provided for the mismatched fragment-index type.

## Short description

Saved cache fragment file is missing.

## Trigger condition

Metadata marks a fragment available, but its expected file is absent during cache loading.

## Explanation

The availability bit is cleared and the loader continues to other fragments.
