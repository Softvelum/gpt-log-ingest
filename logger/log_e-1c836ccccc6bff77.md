# Video filter has no inputs

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-1c836ccccc6bff77

## Log pattern

```text
no filter inputs found, filter graph: %s
```

## Parameters

- Argument 1 — Filter: Complete configured video filter expression, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
no filter inputs found, filter graph: [in]null[out]
```

## Short description

Video filter has no inputs.

## Trigger condition

The parsed video filter graph exposes no input endpoints.

## Explanation

Video filter initialization fails.

## Possible causes

- The expression does not expose a video input.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
