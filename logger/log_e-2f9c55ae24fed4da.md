# VAAPI picture submission could not begin

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-2f9c55ae24fed4da

## Log pattern

```text
%p failed to begin picture, 0x%x
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — VA status: VAAPI status code, integer printed in hexadecimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to begin picture, 0x1
```

## Short description

VAAPI picture submission could not begin.

## Trigger condition

Beginning picture encoding on the input surface returns a non-success status.

## Explanation

Frame encoding fails before its parameter buffers are submitted.

## Possible causes

- The driver rejects beginning an encoding operation with the current context or surface.

## What to check

1. Check the VA status, preceding device errors, and whether the encoder context remains usable.

## Recommended action

Resolve the VA driver or device failure and restart the affected encoder.
