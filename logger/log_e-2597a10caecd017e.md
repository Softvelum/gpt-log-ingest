# DVR thumbnail buffer allocation failed

Product: Nimble Streamer
Component: DVR
Logging function: log_e
Log level: error
Documentation ID: log_e-2597a10caecd017e

## Log pattern

```text
failed to allocate video preview segment buffer, codec=%d size=%d
```

## Parameters

- Argument 1 — Codec: Integer MP4 video codec identifier.
- Argument 2 — Buffer size: Requested preview buffer size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to allocate video preview segment buffer, codec=1635148593 size=65536
```

## Short description

DVR thumbnail buffer allocation failed.

## Trigger condition

The calculated single-frame MP4 preview cannot fit in an allocated buffer.

## Explanation

Thumbnail creation returns failure.

## Possible causes

- The calculated single-frame MP4 preview cannot fit in an allocated buffer.

## What to check

1. Check memory availability and the recorded frame size.

## Recommended action

Check memory availability and the recorded frame size.
