# SCTE-35 private flag is invalid

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-b852ed6c60cd7b3b

## Log pattern

```text
[SCTE35] invalid private_indicator
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] invalid private_indicator
```

## Short description

SCTE-35 private flag is invalid.

## Trigger condition

The private flag is one instead of zero.

## Explanation

Splice-information section parsing stops and reports failure.

## Possible causes

- The section header sets a flag that must be clear.

## What to check

1. Check the relevant field in the upstream SCTE-35 marker.

## Recommended action

Supply an unencrypted valid version-zero splice-information section.
