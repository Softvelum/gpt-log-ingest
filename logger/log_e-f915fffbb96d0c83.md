# Legacy Quick Sync initialization entry is unavailable

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-f915fffbb96d0c83

## Log pattern

```text
failed to load quick sync
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load quick sync
```

## Short description

Legacy Quick Sync initialization entry is unavailable.

## Trigger condition

Legacy session creation finds that the media initialization function was not loaded.

## Explanation

No media session is created.

## Possible causes

- The legacy Intel media runtime library or required entry point was not loaded.

## What to check

1. Check the earlier runtime-library loading diagnostics.

## Recommended action

Install or restore the compatible Intel media runtime and restart transcoding.
