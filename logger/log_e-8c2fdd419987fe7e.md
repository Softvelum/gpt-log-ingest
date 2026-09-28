# LATM frame-length mode unsupported

Product: Nimble Streamer
Component: AAC LATM parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-8c2fdd419987fe7e

## Log pattern

```text
unsupported frameLengthType=%d
```

## Parameters

- Argument 1 — Type: Three-bit LATM frame-length type.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported frameLengthType=1
```

## Short description

LATM frame-length mode unsupported.

## Trigger condition

The LATM frame-length type is not zero.

## Explanation

Stream configuration parsing fails.

## Possible causes

- The LATM frame-length type is not zero.

## What to check

1. Check the LATM frame-length type.

## Recommended action

Use the supported frame-length type zero.
