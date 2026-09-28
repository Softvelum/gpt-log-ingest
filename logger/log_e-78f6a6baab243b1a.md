# SCTE-35 control-word, tier, and command-length area is incomplete

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-78f6a6baab243b1a

## Log pattern

```text
[SCTE35] failed to skip cw_index&tier for
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] failed to skip cw_index&tier for
```

## Short description

SCTE-35 control-word, tier, and command-length area is incomplete.

## Trigger condition

The bit reader cannot consume 32 bits for the control-word, tier, and command-length area.

## Explanation

Splice-information section parsing returns failure at this field.

## Possible causes

- The section ends before the required field can be read or skipped.

## What to check

1. Check the complete SCTE-35 section length and upstream marker bytes.

## Recommended action

Supply a complete correctly sized SCTE-35 section.
