# SCTE-35 table identifier is invalid

Product: Nimble Streamer
Component: SCTE-35 advertisement markers
Logging function: log_e
Log level: error
Documentation ID: log_e-4e8c032d0c2a41bb

## Log pattern

```text
[SCTE35] invalid table_id=%x
```

## Parameters

- Argument 1 — Table identifier: Received 8-bit table identifier, printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] invalid table_id=0
```

## Short description

SCTE-35 table identifier is invalid.

## Trigger condition

The first byte is not hexadecimal FC.

## Explanation

Splice-information section parsing stops and reports failure.

## Possible causes

- The payload is not identified as a splice-information section.

## What to check

1. Check the relevant field in the upstream SCTE-35 marker.

## Recommended action

Supply an unencrypted valid version-zero splice-information section.
