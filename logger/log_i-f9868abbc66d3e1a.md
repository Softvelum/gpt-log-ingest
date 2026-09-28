# NVIDIA context returned for reuse

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-f9868abbc66d3e1a

## Log pattern

```text
nvenc ctx=%p, gpu=%d, flag=%d will be reused
```

## Parameters

- Argument 1 — Context: CUDA context handle, pointer printed in hexadecimal.
- Argument 2 — GPU: Zero-based CUDA device index, integer; no unit.
- Argument 3 — Flags: CUDA context creation flags, integer bit mask printed in decimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
nvenc ctx=0x12340000, gpu=0, flag=0 will be reused
```

## Short description

NVIDIA context returned for reuse.

## Trigger condition

Context sharing is disabled, caching and reuse are enabled, and the released context is found in the in-use map.

## Explanation

The context moves to the free cache for the same GPU and flags instead of being destroyed.
