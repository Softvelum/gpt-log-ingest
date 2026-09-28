# Filter expression could not be parsed

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-ac9c72c99ca3a21d

## Log pattern

```text
failed to parse filter graph: %s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse filter graph: [in]anull[out]
```

## Short description

Filter expression could not be parsed.

## Trigger condition

FFmpeg returns a negative result while parsing the configured audio or video filter expression.

## Explanation

Filter initialization fails before input and output connections are completed.

## Possible causes

- The supplied filter expression cannot be parsed or its filters cannot be created.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
