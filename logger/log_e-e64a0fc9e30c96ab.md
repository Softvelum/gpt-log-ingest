# SCTE-35 encryption algorithm field is incomplete

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-e64a0fc9e30c96ab

## Log pattern

```text
[SCTE35] failed to skip encryption_algorithm
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] failed to skip encryption_algorithm
```

## Short description

SCTE-35 encryption algorithm field is incomplete.

## Trigger condition

The bit reader cannot consume 6 bits for the encryption algorithm field.

## Explanation

Splice-information section parsing returns failure at this field.

## Possible causes

- The section ends before the required field can be read or skipped.

## What to check

1. Check the complete SCTE-35 section length and upstream marker bytes.

## Recommended action

Supply a complete correctly sized SCTE-35 section.
