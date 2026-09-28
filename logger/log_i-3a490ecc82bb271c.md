# Video acceleration device probe failed

Product: Nimble Streamer
Component: Linux video acceleration initialization
Logging function: log_i
Log level: info
Documentation ID: log_i-3a490ecc82bb271c

## Log pattern

```text
failed to get va display for %s err=%d
```

## Parameters

- Argument 1 — Device: Linux render-device path being probed, text.
- Argument 2 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

For display lookup failure the error is the current errno, which may not describe a library-level cause.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get va display for /dev/dri/renderD128 err=5
```

## Short description

Video acceleration device probe failed.

## Trigger condition

During Linux hardware initialization, obtaining its video acceleration display fails.

## Explanation

The device is closed and the next candidate is tried. If all candidates fail, hardware display opening fails.

## Possible causes

- The selected render-device probe failed at this step.

## What to check

1. Check the device, reported error, and whether a later device probe succeeds.

## Recommended action

Resolve access or driver initialization for a suitable render device.
