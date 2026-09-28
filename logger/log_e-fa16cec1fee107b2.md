# RTMP 32-bit network-order integer is truncated

Product: Nimble Streamer
Component: RTMP data parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-fa16cec1fee107b2

## Log pattern

```text
rtmp_read_raw_be_32bits buffer too small[%d]
```

## Parameters

- Argument 1 — Remaining bytes: Bytes remaining after the current parse offset, printed as a signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rtmp_read_raw_be_32bits buffer too small[3]
```

## Short description

RTMP 32-bit network-order integer is truncated.

## Trigger condition

After the existing parse offset, fewer than 4 bytes remain for the 32-bit network-order integer.

## Explanation

Parsing returns failure without consuming the value.

## Possible causes

- The RTMP payload is too short for the expected value.

## What to check

1. Compare the remaining byte count with the expected value size.

## Recommended action

Supply a complete correctly encoded RTMP payload.
