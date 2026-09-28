# Shared CUDA context creation failed

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-7b9a9baf75f88d3d

## Log pattern

```text
failed to create shared ctx for gpu=%d, flag=%d, res=%d
```

## Parameters

- Argument 1 — GPU: Zero-based CUDA device index, integer; no unit.
- Argument 2 — Flags: CUDA context creation flags, integer bit mask printed in decimal; no unit.
- Argument 3 — CUDA status: CUDA driver result code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create shared ctx for gpu=0, flag=0, res=2
```

## Short description

Shared CUDA context creation failed.

## Trigger condition

Creating a new shared CUDA context with zero flags returns an error.

## Explanation

The shared-context request fails before caching a context.

## Possible causes

- The CUDA driver rejects the reported context operation.

## What to check

1. Check the CUDA result, device access, available GPU resources and preceding context diagnostics.

## Recommended action

Correct the driver or resource problem and retry context setup.
