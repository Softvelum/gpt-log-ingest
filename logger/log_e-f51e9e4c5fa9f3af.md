# Quick Sync output surface does not fit its frame

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-f51e9e4c5fa9f3af

## Log pattern

```text
failed to copy planes to output frame, incorrect pitch/height
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to copy planes to output frame, incorrect pitch/height
```

## Short description

Quick Sync output surface does not fit its frame.

## Trigger condition

Either allocated NV12 plane stride is smaller than the returned surface pitch, or the allocated frame height is smaller than the configured crop height.

## Explanation

The output frame is released and pixel copying is skipped; packet processing fails.

## Possible causes

- The returned surface layout does not fit the output frame allocated from the configured crop dimensions.

## What to check

1. Check input dimension changes and the runtime surface pitch against the decoded crop dimensions.

## Recommended action

Use compatible video dimensions and runtime output layout, or select another decoder.
