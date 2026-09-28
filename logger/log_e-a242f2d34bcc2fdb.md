# GPU identifier query disabled automatic assignment

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-a242f2d34bcc2fdb

## Log pattern

```text
failed to get uuid for gpu=%d, err=%d, nvenc gpu auto assigning disabled
```

## Parameters

- Argument 1 — GPU: Zero-based CUDA device index, integer; no unit.
- Argument 2 — CUDA status: CUDA driver result code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get uuid for gpu=0, err=2, nvenc gpu auto assigning disabled
```

## Short description

GPU identifier query disabled automatic assignment.

## Trigger condition

The CUDA UUID query fails for an enumerated GPU during management-device mapping.

## Explanation

The temporary GPU mapping is cleared and automatic load-based assignment is not enabled; context-cache initialization can still continue.

## Possible causes

- The CUDA driver cannot return the device UUID.

## What to check

1. Check the CUDA status and driver access for this GPU.

## Recommended action

Repair GPU identification support or explicitly select a GPU.
