# NVIDIA input buffer lock failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-6a5287bfdf843dd2

## Log pattern

```text
failed to lock input buffer encoder=%p, status=%d
```

## Parameters

- Argument 1 — Encoder: NVIDIA encoder session handle, pointer printed in hexadecimal.
- Argument 2 — Status: NVIDIA encoder API result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to lock input buffer encoder=0x12340000, status=8
```

## Short description

NVIDIA input buffer lock failed.

## Trigger condition

Locking the chosen input buffer for writing returns an error.

## Explanation

The frame write returns failure before pixels are copied.

## Possible causes

- The NVIDIA encoder API rejects this buffer or picture operation.

## What to check

1. Check the buffer/session state and earlier errors.

## Recommended action

Correct the reported driver/resource or encoder-state problem before retrying.
