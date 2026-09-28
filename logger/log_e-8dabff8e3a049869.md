# Quick Sync HEVC VPS could not be parsed

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-8dabff8e3a049869

## Log pattern

```text
failed to parse annexb vps
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse annexb vps
```

## Short description

Quick Sync HEVC VPS could not be parsed.

## Trigger condition

The Annex B converter rejects the VPS returned by the initialized HEVC encoder.

## Explanation

Encoder initialization returns failure without a complete HEVC output configuration.

## Possible causes

- The returned parameter-set buffer is too short or contains an empty NAL unit.

## What to check

1. Check the HEVC encoder output headers and media runtime compatibility.

## Recommended action

Use a runtime and encoding configuration that produce valid HEVC parameter sets.
