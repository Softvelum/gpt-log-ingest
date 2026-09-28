# VAAPI input image unmapping failed

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-5e6d6d7bb10ffc7f

## Log pattern

```text
%p failed to unmap buffer, 0x%x
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — VA status: VAAPI status code, integer printed in hexadecimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to unmap buffer, 0x1
```

## Short description

VAAPI input image unmapping failed.

## Trigger condition

Unmapping the image buffer after copying input pixels returns a non-success status.

## Explanation

Frame encoding fails before the image is destroyed and the surface is synchronized.

## Possible causes

- The driver cannot finish the image-buffer mapping operation.

## What to check

1. Check the VA status, preceding device errors, and whether the encoder context remains usable.

## Recommended action

Resolve the VA driver or device failure and restart the affected encoder.
