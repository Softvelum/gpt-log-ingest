# SVT HEVC parameters were rejected

Product: Nimble Streamer
Component: SVT HEVC encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-1e56d44acc2d03d1

## Log pattern

```text
failed to set encode params, err=%d
```

## Parameters

- Argument 1 — Encoder status: SVT HEVC result code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set encode params, err=-1
```

## Short description

SVT HEVC parameters were rejected.

## Trigger condition

Applying the completed encoder configuration returns an error.

## Explanation

The handle is destroyed and cleared; encoder initialization fails.

## Possible causes

- The library rejects the requested dimensions, rate control, profile or other encoder settings.

## What to check

1. Check the SVT status and configured video/encoder parameters.

## Recommended action

Correct incompatible encoding parameters.
