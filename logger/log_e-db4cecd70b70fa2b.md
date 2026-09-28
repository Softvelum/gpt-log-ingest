# Codec is unsupported by native Quick Sync encoding

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-db4cecd70b70fa2b

## Log pattern

```text
unsupported quick sync codec=%d
```

## Parameters

- Argument 1 — Codec: FFmpeg codec identifier, integer enumeration value; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported quick sync codec=167
```

## Short description

Codec is unsupported by native Quick Sync encoding.

## Trigger condition

Encoder initialization receives a codec other than H.264 or HEVC.

## Explanation

Initialization returns failure before media parameters are prepared.

## Possible causes

- The selected output codec is outside the native encoder support list.

## What to check

1. Check the numeric FFmpeg codec identifier and chosen encoder.

## Recommended action

Choose H.264 or HEVC for this encoder, or use another backend.
