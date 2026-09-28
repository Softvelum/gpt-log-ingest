# Modern NVIDIA preset retrieval failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-e1dc9fa946ed66d8

## Log pattern

```text
failed to get NVENC preset configEx session=%p nvStatus=%d
```

## Parameters

- Argument 1 — Encoder: NVIDIA encoder session handle, pointer printed in hexadecimal.
- Argument 2 — Status: NVIDIA encoder API result, integer; no unit.

Both occurrences use the extended preset query; one is the initial path and one follows a failed legacy query.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get NVENC preset configEx session=0x12340000 nvStatus=8
```

## Short description

Modern NVIDIA preset retrieval failed.

## Trigger condition

Retrieving a modern preset and tuning configuration returns an error, either for AV1/an explicitly modern preset or after a legacy-preset fallback.

## Explanation

Encoder initialization returns failure.

## Possible causes

- The driver rejects the codec, preset or tuning configuration query.

## What to check

1. Check the result code, selected preset/tuning and driver compatibility.

## Recommended action

Select supported preset/tuning values or use a compatible driver.

## Related log messages

- [Legacy NVIDIA preset lookup is falling back](log_v-bfd041281ea39d63.md)
