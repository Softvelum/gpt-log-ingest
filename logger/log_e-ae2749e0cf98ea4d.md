# NVIDIA sequence-header retrieval failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ae2749e0cf98ea4d

## Log pattern

```text
failed to get sps/pps encoder=%p, status=%d
```

## Parameters

- Argument 1 — Encoder: NVIDIA encoder session handle, pointer printed in hexadecimal.
- Argument 2 — Status: NVIDIA encoder API result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get sps/pps encoder=0x12340000, status=8
```

## Short description

NVIDIA sequence-header retrieval failed.

## Trigger condition

The initialized encoder fails to return its sequence parameters into the header buffer.

## Explanation

Encoder setup returns failure before publishing codec configuration. The operation also serves HEVC and compiled AV1 despite the message naming SPS/PPS.

## Possible causes

- The NVIDIA encoder cannot provide its sequence header.

## What to check

1. Check the result and preceding codec/initialization diagnostics.

## Recommended action

Restore valid encoder state and retry.
