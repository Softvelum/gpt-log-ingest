# Shared hardware-device creation failed

Product: Nimble Streamer
Component: Shared hardware devices
Logging function: log_e
Log level: error
Documentation ID: log_e-b63439b4ae5fc85f

## Log pattern

```text
failed to create shared hw device ctx type=%d dev=%s err=%d
```

## Parameters

- Argument 1 — Device type: FFmpeg hardware-device type identifier, integer; no unit.
- Argument 2 — Device: Hardware device name or device node, text; VAAPI with no explicit name first attempts Intel render-device discovery.
- Argument 3 — Error: FFmpeg negative error code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create shared hw device ctx type=3 dev=/dev/dri/renderD128 err=-22
```

## Short description

Shared hardware-device creation failed.

## Trigger condition

No cached context exists for the requested device type/name, and FFmpeg hardware-device creation returns a negative error.

## Explanation

The device-context request returns no context.

## Possible causes

- FFmpeg cannot open or initialize the requested hardware device.

## What to check

1. Check the device name, type, permissions, driver support and negative FFmpeg error.

## Recommended action

Restore device accessibility and compatible driver support.
