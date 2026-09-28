# Xilinx transcoding devices found

Product: Nimble Streamer
Component: Transcoder orchestration
Logging function: log_i
Log level: info
Documentation ID: log_i-a41f46df0e85e37f

## Log pattern

```text
found %d xilinx devices
```

## Parameters

- Argument 1 — Devices: Enumerated Xilinx device count, integer devices.

## Log examples

Illustrative examples; these are not captured server logs.

```text
found 2 xilinx devices
```

## Short description

Xilinx transcoding devices found.

## Trigger condition

Xilinx media support is available and device enumeration reports more than zero devices.

## Explanation

The listed number of devices will be passed to initialization with the transcoding hardware image.
