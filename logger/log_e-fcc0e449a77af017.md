# Requested NVIDIA device index is unavailable

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-fcc0e449a77af017

## Log pattern

```text
no nvenc device %d found
```

## Parameters

- Argument 1 — GPU: Zero-based CUDA device index, integer; no unit.

The two occurrences cover private context creation and shared context access.

## Log examples

Illustrative examples; these are not captured server logs.

```text
no nvenc device 0 found
```

## Short description

Requested NVIDIA device index is unavailable.

## Trigger condition

Context creation or shared-context retrieval receives a negative GPU index or an index beyond the enumerated device list.

## Explanation

The context request returns failure before calling the driver.

## Possible causes

- The requested index does not identify an enumerated CUDA device.

## What to check

1. Compare the selected GPU index with device discovery messages.

## Recommended action

Select an available GPU index.
