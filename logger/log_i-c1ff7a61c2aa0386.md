# Inactive overlay is marked as ended

Product: Nimble Streamer
Component: Video filtering
Logging function: log_i
Log level: info
Documentation ID: log_i-c1ff7a61c2aa0386

## Log pattern

```text
set eof stream=%s with main_stream=%s, filter=%s
```

## Parameters

- Argument 1 — Input: Input stream name, text.
- Argument 2 — Main input: Main video stream name, text.
- Argument 3 — Filter: Configured filter identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
set eof stream=live/overlay with main_stream=live/camera, filter=overlay1
```

## Short description

Inactive overlay is marked as ended.

## Trigger condition

Overlay timestamp adjustment is enabled, an eligible secondary input has no new frame, and the main input is more than five seconds ahead of its positive adjusted last timestamp.

## Explanation

End-of-input is submitted to the filter for this overlay so processing can proceed without waiting for it; submission failure stops filtering.

## Possible causes

- The overlay has stopped delivering frames while the main stream continues.

## What to check

1. Check availability of the named secondary source and whether its last frame is stale.

## Recommended action

Restore the overlay source if it should remain active; returning frames trigger a filter rebuild.
