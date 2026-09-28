# VPX encoder initialization failed

Product: Nimble Streamer
Component: VP8 and VP9 encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-3b2b3ce5b35639aa

## Log pattern

```text
failed to init vpx encoder: %s
```

## Parameters

- Argument 1 — Codec error: libvpx description of the returned error, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to init vpx encoder: Invalid parameter
```

## Short description

VPX encoder initialization failed.

## Trigger condition

libvpx rejects encoder initialization after dimensions, time base, and configured options have been applied.

## Explanation

No usable VPX encoder is created for the first frame.

## Possible causes

- The requested encoding configuration or encoder ABI is incompatible with the loaded library, or initialization resources are unavailable.

## What to check

1. Check the error description, video dimensions, and encoding parameters.

## Recommended action

Correct the configuration or library compatibility problem and restart encoding.
