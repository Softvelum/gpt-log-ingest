# HLS key downloaded despite already being cached

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-63d7fbd2369ebf6a

## Log pattern

```text
existing encryption key requested again, uri=%s, hls camera s=%d
```

## Parameters

- Argument 1 — key\_uri: Key URL already present in the cache.
- Argument 2 — socket: Origin connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
existing encryption key requested again, uri=https://origin.example/key.bin, hls camera s=42
```

## Short description

HLS key downloaded despite already being cached.

## Trigger condition

The downloaded key has the expected 16-byte length, but its current URI already exists in the key map.

## Explanation

The key setter returns false instead of replacing the cached key. Its caller then requests shutdown. Normal segment selection is intended to request a key only when absent.

## Possible causes

- A duplicate key request reached insertion for an already cached URI.

## What to check

1. Check earlier key requests and URI reuse for this HLS input.
