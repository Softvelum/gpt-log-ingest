# SCTE-35 section syntax flag is incomplete

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-e0f23c8f976ddca4

## Log pattern

```text
[SCTE35] failed to read section_syntax_indicator
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] failed to read section_syntax_indicator
```

## Short description

SCTE-35 section syntax flag is incomplete.

## Trigger condition

The bit reader cannot consume 1 bit for the section syntax flag.

## Explanation

Splice-information section parsing returns failure at this field.

## Possible causes

- The section ends before the required field can be read or skipped.

## What to check

1. Check the complete SCTE-35 section length and upstream marker bytes.

## Recommended action

Supply a complete correctly sized SCTE-35 section.
