# Quick Sync device creation failed

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-7f25ebed4cbc9da5

## Log pattern

```text
failed to create quicksync hwdevice ctx
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create quicksync hwdevice ctx
```

## Short description

Quick Sync device creation failed.

## Trigger condition

FFmpeg returns a negative result while creating the shared Quick Sync hardware device with automatic device selection.

## Explanation

No shared device reference is returned; the requesting hardware operation cannot use this context.

## Possible causes

- The installed media runtime or selected device cannot initialize a Quick Sync device.

## What to check

1. Check the preceding FFmpeg diagnostics and availability of the intended Intel media device.

## Recommended action

Restore a usable media runtime and device, then restart the affected transcoding operation.
