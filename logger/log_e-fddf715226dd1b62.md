# SCTE-35 reserved header bits is incomplete

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-fddf715226dd1b62

## Log pattern

```text
[SCTE35] failed to skip reserved
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] failed to skip reserved
```

## Short description

SCTE-35 reserved header bits is incomplete.

## Trigger condition

The bit reader cannot consume 2 bits for the reserved header bits.

## Explanation

Splice-information section parsing returns failure at this field.

## Possible causes

- The section ends before the required field can be read or skipped.

## What to check

1. Check the complete SCTE-35 section length and upstream marker bytes.

## Recommended action

Supply a complete correctly sized SCTE-35 section.
