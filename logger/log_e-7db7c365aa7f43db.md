# SCTE-35 section syntax flag is invalid

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-7db7c365aa7f43db

## Log pattern

```text
[SCTE35] invalid section_syntax_indicator
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] invalid section_syntax_indicator
```

## Short description

SCTE-35 section syntax flag is invalid.

## Trigger condition

The section syntax flag is one instead of zero.

## Explanation

Splice-information section parsing stops and reports failure.

## Possible causes

- The section header has an unsupported syntax flag.

## What to check

1. Check the relevant field in the upstream SCTE-35 marker.

## Recommended action

Supply an unencrypted valid version-zero splice-information section.
