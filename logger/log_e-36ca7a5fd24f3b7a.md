# Audio filter output support is unavailable

Product: Nimble Streamer
Component: Audio and video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-36ca7a5fd24f3b7a

## Log pattern

```text
failed to get abuffersink definition, %s
```

## Parameters

- Argument 1 — Filter: Complete configured filter expression, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get abuffersink definition, [in]anull[out]
```

## Short description

Audio filter output support is unavailable.

## Trigger condition

FFmpeg cannot find its audio-buffer sink filter definition.

## Explanation

Audio filter initialization fails.

## Possible causes

- The loaded library lacks the required audio sink filter.

## What to check

1. Check the affected filter expression, its input/output labels, and accompanying FFmpeg diagnostics.

## Recommended action

Correct the reported filter setup or frame-processing failure.
