# NVIDIA encoded-output unlock failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-e40890775f1afeec

## Log pattern

```text
failed to unlock output bitstream encoder=%p, status=%d
```

## Parameters

- Argument 1 — Encoder: NVIDIA encoder session handle, pointer printed in hexadecimal.
- Argument 2 — Status: NVIDIA encoder API result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to unlock output bitstream encoder=0x12340000, status=8
```

## Short description

NVIDIA encoded-output unlock failed.

## Trigger condition

Unlocking an output bitstream after copying its data returns an error.

## Explanation

The frame write returns failure before freeing the slot and sending the packet downstream.

## Possible causes

- The NVIDIA encoder API rejects this buffer or picture operation.

## What to check

1. Check driver and output-buffer state.

## Recommended action

Correct the reported driver/resource or encoder-state problem before retrying.
