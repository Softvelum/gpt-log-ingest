# VAAPI encoding configuration creation failed

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-b0d09a54ae99d53e

## Log pattern

```text
%p failed to create config, 0x%x
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — VA status: VAAPI status code, integer printed in hexadecimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to create config, 0x1
```

## Short description

VAAPI encoding configuration creation failed.

## Trigger condition

The VA driver returns a non-success status while creating the YUV420 encoding configuration for the selected profile and rate control.

## Explanation

Encoder initialization returns failure at this stage.

## Possible causes

- The selected profile, entry point, or rate control is unsupported by the driver.

## What to check

1. Check the VA status, selected profile and dimensions, and device resource availability.

## Recommended action

Correct unsupported settings or restore device resources, then restart encoding.
