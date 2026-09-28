# Quick Sync decoder initialization failed

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-9ebdee8f7ff20feb

## Log pattern

```text
failed to init decoder, status=%d
```

## Parameters

- Argument 1 — Status: Signed integer status returned by the Intel media runtime; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to init decoder, status=-3
```

## Short description

Quick Sync decoder initialization failed.

## Trigger condition

Initializing the native decoder returns any status other than success, including a warning.

## Explanation

No decoder output surface pool is prepared and initialization fails.

## Possible causes

- The media runtime does not accept the parsed parameters with 8-bit system-memory output and one asynchronous operation.

## What to check

1. Check the status, parsed input characteristics, and decoder support.

## Recommended action

Correct the unsupported input or runtime configuration and restart decoding.
