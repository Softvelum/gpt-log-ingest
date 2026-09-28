# Audio filter has no inputs

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-167ddfa278413bba

## Log pattern

```text
no inputs found for filter graph: %s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
no inputs found for filter graph: [in]anull[out]
```

## Short description

Audio filter has no inputs.

## Trigger condition

The parsed audio filter graph has no unconnected input endpoints.

## Explanation

Audio filter initialization fails.

## Possible causes

- The expression does not expose the required input.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
