# VAAPI profile query failed

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ccc94baee37cdc44

## Log pattern

```text
%p failed to get supported profiles, 0x%x
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — VA status: VAAPI status code, integer printed in hexadecimal; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to get supported profiles, 0x1
```

## Short description

VAAPI profile query failed.

## Trigger condition

The VA driver returns a non-success status when supported profiles are queried.

## Explanation

The profile-query operation returns failure without a usable result.

## Possible causes

- The VA display or driver cannot provide its supported profile list.

## What to check

1. Check the status and VA device initialization diagnostics.

## Recommended action

Restore a working VA driver and device connection.
