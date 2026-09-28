# SVT HEVC encoder initialization failed

Product: Nimble Streamer
Component: SVT HEVC encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-d535d6d81bf51eb2

## Log pattern

```text
failed to init encoder, err=%d
```

## Parameters

- Argument 1 — Encoder status: SVT HEVC result code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to init encoder, err=-1
```

## Short description

SVT HEVC encoder initialization failed.

## Trigger condition

Starting the configured SVT HEVC encoder returns an error.

## Explanation

The handle is destroyed and cleared; encoder initialization fails.

## Possible causes

- The library cannot initialize the configured encoder.

## What to check

1. Check the SVT status, encoder settings and memory/processor resources.

## Recommended action

Correct the initialization problem and retry.
