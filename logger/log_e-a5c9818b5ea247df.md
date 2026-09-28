# Audio conversion channel layout copy failed

Product: Nimble Streamer
Component: Audio frame preparation
Logging function: log_e
Log level: error
Documentation ID: log_e-a5c9818b5ea247df

## Log pattern

```text
failed to copy channel layout
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to copy channel layout
```

## Short description

Audio conversion channel layout copy failed.

## Trigger condition

Copying the input channel layout into the conversion frame returns a nonzero result.

## Explanation

The input frame write fails before conversion.

## Possible causes

- The stated frame allocation, layout copy, or sample conversion operation failed.

## What to check

1. Check the input audio format, channel layout, and accompanying FFmpeg diagnostics.

## Recommended action

Resolve the reported frame-preparation failure.
