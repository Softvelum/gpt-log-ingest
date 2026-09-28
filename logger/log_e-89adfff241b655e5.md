# Explicit AAC sample frequency is unsupported

Product: Nimble Streamer
Component: AAC media processing
Logging function: log_e
Log level: error
Documentation ID: log_e-89adfff241b655e5

## Log pattern

```text
[AM] wrong samplingFrequencyIndex %d
```

## Parameters

- Argument 1 — Frequency index: Integer AAC frequency index; this condition is 15.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[AM] wrong samplingFrequencyIndex 15
```

## Short description

Explicit AAC sample frequency is unsupported.

## Trigger condition

The AAC configuration uses frequency index 15 during DVR initialization or fragmented-audio conversion to MPEG-TS.

## Explanation

The operation returns failure instead of interpreting the explicit frequency.

## Possible causes

- The AAC configuration uses frequency index 15 during DVR initialization or fragmented-audio conversion to MPEG-TS.

## What to check

1. Supply AAC configuration with a supported indexed sample frequency.

## Recommended action

Supply AAC configuration with a supported indexed sample frequency.
