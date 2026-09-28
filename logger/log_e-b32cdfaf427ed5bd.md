# SLDP video is already selected

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-b32cdfaf427ed5bd

## Log pattern

```text
video is already requested, sn=%d
```

## Parameters

- Argument 1 — Existing video number: Client stream number already assigned to this video selection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
video is already requested, sn=1
```

## Short description

SLDP video is already selected.

## Trigger condition

A duration-free Play request merges with an existing selection for the same source buffer and offset, but video is already enabled there.

## Explanation

The duplicate selection is logged and processing continues. The existing stream number is retained rather than adding another copy.

## Possible causes

- A duration-free Play request merges with an existing selection for the same source buffer and offset, but video is already enabled there.

## What to check

1. Check duplicate video requests for the same stream and offset.

## Recommended action

Avoid requesting the same media selection twice.
