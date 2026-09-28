# VAAPI surface synchronization failed

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-3172eaa33fbd1bf6

## Log pattern

```text
%p failed to sync surface, 0x%x
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — VA status: VAAPI status code, integer printed in hexadecimal; no unit.

The same message occurs after copying pixels into the input surface and after submitting the encoded picture.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to sync surface, 0x1
```

## Short description

VAAPI surface synchronization failed.

## Trigger condition

Synchronizing the input surface returns a non-success VA status.

## Explanation

Before submission, failure prevents encoding after input pixels were copied. After submission, failure prevents mapping and delivering encoded output.

## Possible causes

- The VA surface operation could not finish successfully.

## What to check

1. Check the status and preceding image-copy or encoding messages to locate the failing stage.

## Recommended action

Resolve the device or driver fault and restart the encoder.
