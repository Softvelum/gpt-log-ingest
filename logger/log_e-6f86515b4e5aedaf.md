# NVIDIA bitstream buffer allocation failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-6f86515b4e5aedaf

## Log pattern

```text
failed to create output bitstream encoder=%p, status=%d
```

## Parameters

- Argument 1 — Encoder: NVIDIA encoder session handle, pointer printed in hexadecimal.
- Argument 2 — Status: NVIDIA encoder API result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create output bitstream encoder=0x12340000, status=8
```

## Short description

NVIDIA bitstream buffer allocation failed.

## Trigger condition

Creating the paired one-megabyte encoded-output buffer returns an error.

## Explanation

The current frame write returns failure before the slot is fully initialized.

## Possible causes

- The NVIDIA encoder API rejects this buffer or picture operation.

## What to check

1. Check encoder memory resources and the result code.

## Recommended action

Correct the reported driver/resource or encoder-state problem before retrying.
