# Audio filter graph allocation failed

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-ca0082a7af2c0f93

## Log pattern

```text
failed to create filter graph, out of memory, %s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create filter graph, out of memory, [in]anull[out]
```

## Short description

Audio filter graph allocation failed.

## Trigger condition

Allocating the audio filter graph returns no graph.

## Explanation

Audio filter initialization fails.

## Possible causes

- Audio filter graph allocation could not obtain storage.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
