# CUDA context detach failed

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-2d6aa9a181db886b

## Log pattern

```text
failed to detach nvenc ctx from thread, gpu=%d, flag=%d, res=%d
```

## Parameters

- Argument 1 — GPU: Zero-based CUDA device index, integer; no unit.
- Argument 2 — Flags: CUDA context creation flags, integer bit mask printed in decimal; no unit.
- Argument 3 — CUDA status: CUDA driver result code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to detach nvenc ctx from thread, gpu=0, flag=0, res=2
```

## Short description

CUDA context detach failed.

## Trigger condition

After creating a private CUDA context, removing it from the calling thread current-context stack returns an error.

## Explanation

The creation request returns failure despite having reached context allocation.

## Possible causes

- The CUDA driver rejects the reported context operation.

## What to check

1. Check the CUDA result, device access, available GPU resources and preceding context diagnostics.

## Recommended action

Correct the driver or resource problem and retry context setup.
