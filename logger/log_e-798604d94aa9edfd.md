# RTMP number is truncated

Product: Nimble Streamer
Component: RTMP data parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-798604d94aa9edfd

## Log pattern

```text
rtmp_read_number buffer too small[%d]
```

## Parameters

- Argument 1 — Remaining bytes: Bytes remaining after the current parse offset, printed as a signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rtmp_read_number buffer too small[8]
```

## Short description

RTMP number is truncated.

## Trigger condition

After the existing parse offset, fewer than 9 bytes remain for the number.

## Explanation

Parsing returns failure without consuming the value.

## Possible causes

- The RTMP payload is too short for the expected value.

## What to check

1. Compare the remaining byte count with the expected value size.

## Recommended action

Supply a complete correctly encoded RTMP payload.
