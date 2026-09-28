# Native NVIDIA decoder GPU selected

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-a44b7ac1d2331ce5

## Log pattern

```text
assigned nvenc decoder to gpu=%d, stream='%s'
```

## Parameters

- Argument 1 — GPU: Zero-based CUDA device index, integer; no unit.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
assigned nvenc decoder to gpu=0, stream='live/camera'
```

## Short description

Native NVIDIA decoder GPU selected.

## Trigger condition

The native NVIDIA decoder requests automatic GPU assignment.

## Explanation

The chosen index is used for its shared or private CUDA context. Selection compares decoder utilization with recent-assignment penalties; unavailable balancing or query failures fall back to GPU zero.
