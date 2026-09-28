# GPU decoder query disabled automatic assignment

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-0b5af32effbdc1bb

## Log pattern

```text
failed to get decoder utilization, gpu=%d, err=%d, nvenc gpu auto assigning disabled
```

## Parameters

- Argument 1 — GPU: Zero-based CUDA device index, integer; no unit.
- Argument 2 — Management status: NVIDIA management-library result code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get decoder utilization, gpu=0, err=3, nvenc gpu auto assigning disabled
```

## Short description

GPU decoder query disabled automatic assignment.

## Trigger condition

A startup probe for decoder utilization returns a management-library error.

## Explanation

The temporary GPU mapping is cleared and automatic load-based assignment is not enabled; other context initialization may continue.

## Possible causes

- The selected device or driver cannot provide decoder utilization.

## What to check

1. Check the management status and whether the device supports utilization reporting.

## Recommended action

Restore utilization reporting or explicitly select a GPU.
