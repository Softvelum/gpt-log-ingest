# Audio filter has no outputs

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-1ae0da357fcf65dc

## Log pattern

```text
no outputs found for filter graph: %s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
no outputs found for filter graph: [in]anull[out]
```

## Short description

Audio filter has no outputs.

## Trigger condition

The parsed audio filter graph has no unconnected output endpoints.

## Explanation

Audio filter initialization fails.

## Possible causes

- The expression does not expose an output.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
