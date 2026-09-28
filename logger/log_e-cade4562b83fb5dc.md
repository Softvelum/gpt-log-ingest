# Automatic GPU selection found no candidate

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-cade4562b83fb5dc

## Log pattern

```text
nvenc gpu auto assigning failed
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
nvenc gpu auto assigning failed
```

## Short description

Automatic GPU selection found no candidate.

## Trigger condition

Automatic assignment is enabled, but the device comparison finishes without selecting a mapped GPU.

## Explanation

GPU zero is returned as the fallback for encoder, native decoder or FFmpeg decoder assignment.

## Possible causes

- No mapped GPU is available to the selection pass.

## What to check

1. Check device discovery, management mapping and device-release timing.

## Recommended action

Restore a valid GPU mapping or specify an available GPU explicitly.
