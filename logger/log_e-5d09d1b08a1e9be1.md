# VAAPI slice type is unsupported

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-5d09d1b08a1e9be1

## Log pattern

```text
%p invalid slice type=%d
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — Slice type: Internal slice-type integer: P is 5, B is 6, I is 7; no unit.

Two H.264 checks validate picture and slice preparation. The VP8 check accepts only I and P.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 invalid slice type=9
```

## Short description

VAAPI slice type is unsupported.

## Trigger condition

A requested slice type falls outside the accepted set in picture or slice preparation.

## Explanation

Frame encoding returns failure before picture submission.

## Possible causes

- The internal frame request is not I, P, or B for H.264, or not I or P for VP8.

## What to check

1. Retain the codec, reported type, and preceding frame-ordering diagnostics.

## Recommended action

Restart the encoder and investigate a reproducible invalid frame-type request.
