# SCTE-35 presentation timestamp adjustment is incomplete

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-b28df4fb380cf5e7

## Log pattern

```text
[SCTE35] failed to read pts_adjustment
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] failed to read pts_adjustment
```

## Short description

SCTE-35 presentation timestamp adjustment is incomplete.

## Trigger condition

The bit reader cannot consume 33 bits for the presentation timestamp adjustment.

## Explanation

Splice-information section parsing returns failure at this field.

## Possible causes

- The section ends before the required field can be read or skipped.

## What to check

1. Check the complete SCTE-35 section length and upstream marker bytes.

## Recommended action

Supply a complete correctly sized SCTE-35 section.
