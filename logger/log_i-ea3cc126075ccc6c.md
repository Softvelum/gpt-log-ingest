# Cached NVIDIA context checked out

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-ea3cc126075ccc6c

## Log pattern

```text
get cached nvenc ctx=%p (gpu=%d flag=%d)
```

## Parameters

- Argument 1 — Context: CUDA context handle, pointer printed in hexadecimal.
- Argument 2 — GPU: Zero-based CUDA device index, integer; no unit.
- Argument 3 — Flags: CUDA context creation flags, integer bit mask printed in decimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
get cached nvenc ctx=0x12340000 (gpu=0 flag=0)
```

## Short description

Cached NVIDIA context checked out.

## Trigger condition

Context caching is enabled and a free context exists for the requested GPU and flags.

## Explanation

The context is removed from the free list and returned; if reuse is enabled, it is tracked as in use.
