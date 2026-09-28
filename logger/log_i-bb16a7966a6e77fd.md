# Named NVIDIA device discovered

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-bb16a7966a6e77fd

## Log pattern

```text
%s nvenc device %d found
```

## Parameters

- Argument 1 — Device name: Device name returned by the CUDA driver, text.
- Argument 2 — GPU: Zero-based CUDA device index, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
NVIDIA GPU nvenc device 0 found
```

## Short description

Named NVIDIA device discovered.

## Trigger condition

A CUDA device is retrieved and its name query succeeds.

## Explanation

The device is added to the available device list.
