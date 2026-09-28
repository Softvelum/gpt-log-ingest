# SCTE-35 protocol version is incomplete

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-9e2f6c8d4879ea8e

## Log pattern

```text
[SCTE35] failed to read protocol_version
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] failed to read protocol_version
```

## Short description

SCTE-35 protocol version is incomplete.

## Trigger condition

The bit reader cannot consume 8 bits for the protocol version.

## Explanation

Splice-information section parsing returns failure at this field.

## Possible causes

- The section ends before the required field can be read or skipped.

## What to check

1. Check the complete SCTE-35 section length and upstream marker bytes.

## Recommended action

Supply a complete correctly sized SCTE-35 section.
