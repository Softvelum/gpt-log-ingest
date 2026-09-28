# Video filter output could not be created

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-ef2a9e650fec9a20

## Log pattern

```text
failed to create buffersink, filter graph: %s
```

## Parameters

- Argument 1 — Filter: Complete configured video filter expression, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create buffersink, filter graph: [in]null[out]
```

## Short description

Video filter output could not be created.

## Trigger condition

Creating a sink for a parsed video output fails.

## Explanation

Video filter initialization fails.

## Possible causes

- FFmpeg rejected creation of the video output sink.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
