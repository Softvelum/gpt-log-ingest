# NVIDIA encoding session could not open

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-bf605aeef2809c4e

## Log pattern

```text
failed to open nvenc encode session, %d
```

## Parameters

- Argument 1 — Status: NVIDIA encoder API result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open nvenc encode session, 8
```

## Short description

NVIDIA encoding session could not open.

## Trigger condition

Opening an encoding session on the obtained CUDA context with the requested API version returns an error.

## Explanation

Encoder initialization returns failure before loading preset configuration.

## Possible causes

- The driver cannot open the requested encoder session.

## What to check

1. Check the NVIDIA result, driver/API compatibility and encoder-session capacity.

## Recommended action

Restore compatible driver support or available encoding capacity.
