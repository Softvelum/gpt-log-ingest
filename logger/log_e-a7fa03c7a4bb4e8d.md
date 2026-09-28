# VAAPI quality-level buffer creation failed

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-a7fa03c7a4bb4e8d

## Log pattern

```text
%p failed to create quality level params buffer, 0x%x
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — VA status: VAAPI status code, integer printed in hexadecimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to create quality level params buffer, 0x1
```

## Short description

VAAPI quality-level buffer creation failed.

## Trigger condition

A nonzero quality option is selected. The driver returns a non-success status when asked to create the corresponding parameter buffer.

## Explanation

Encoder initialization fails.

## Possible causes

- The VA driver cannot create this encoding parameter buffer.

## What to check

1. Check the status, the corresponding encoding options, and device resource availability.

## Recommended action

Correct incompatible encoding options or restore the VA driver and device, then restart encoding.
