# CUDA device lookup failed

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-a90cb61d8c3e15a3

## Log pattern

```text
failed to get cuda device %d
```

## Parameters

- Argument 1 — GPU: Zero-based CUDA device index, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get cuda device 0
```

## Short description

CUDA device lookup failed.

## Trigger condition

Enumerating CUDA devices succeeds, but retrieving the device at the reported index returns an error.

## Explanation

Device enumeration stops and NVIDIA context-manager initialization fails.

## Possible causes

- The CUDA driver cannot retrieve an enumerated device.

## What to check

1. Check device availability and CUDA driver diagnostics for the index.

## Recommended action

Restore CUDA device access and retry initialization.

## Related log messages

- [CUDA device enumeration failed](log_e-cb54249450bb6816.md)
