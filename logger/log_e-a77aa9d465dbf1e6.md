# LATM multiple programs or layers unsupported

Product: Nimble Streamer
Component: AAC LATM parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-a77aa9d465dbf1e6

## Log pattern

```text
unsupported numProgram=%d numLayer=%d
```

## Parameters

- Argument 1 — Program index: LATM number-of-programs field, representing count minus one.
- Argument 2 — Layer index: LATM number-of-layers field, representing count minus one.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported numProgram=1 numLayer=0
```

## Short description

LATM multiple programs or layers unsupported.

## Trigger condition

The LATM program or layer index is nonzero.

## Explanation

Stream configuration parsing fails; the reader accepts only one program with one layer.

## Possible causes

- The LATM program or layer index is nonzero.

## What to check

1. Check the LATM program and layer counts.

## Recommended action

Supply single-program, single-layer LATM.
