# VAAPI video surfaces creation failed

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-fb7d12a1fc813d57

## Log pattern

```text
%p failed to create surfaces, 0x%x
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — VA status: VAAPI status code, integer printed in hexadecimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to create surfaces, 0x1
```

## Short description

VAAPI video surfaces creation failed.

## Trigger condition

The VA driver returns a non-success status while creating four YUV420 surfaces at dimensions aligned to 16 pixels.

## Explanation

Encoder initialization returns failure at this stage.

## Possible causes

- The device cannot allocate surfaces for the requested dimensions or format.

## What to check

1. Check the VA status, selected profile and dimensions, and device resource availability.

## Recommended action

Correct unsupported settings or restore device resources, then restart encoding.
