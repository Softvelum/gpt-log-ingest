# HLS fragmented MP4 segment encryption fails

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-bf3fe1b6ca858749

## Log pattern

```text
failed to encrypt fmp4 segment='%s'
```

## Parameters

- Argument 1 — Cache key: Application/stream path and generated segment or part name used as the cache identity.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to encrypt fmp4 segment='/live/camera/v_100_90000_10.fmp4'
```

## Short description

HLS fragmented MP4 segment encryption fails.

## Trigger condition

DRM preparation fails while processing the serialized segment.

## Explanation

The reserved cache item is cancelled and released; the item is not published.

## Possible causes

- DRM preparation fails while processing the serialized segment.

## What to check

1. Check the matching DRM key/preparation errors and encryption configuration.

## Recommended action

Restore valid DRM key material and successful encryption preparation.
