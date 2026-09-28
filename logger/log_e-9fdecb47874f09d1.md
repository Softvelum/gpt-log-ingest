# SCTE-35 splice point cannot be read

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-9fdecb47874f09d1

## Log pattern

```text
[SCTE35] failed to read splice point for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] failed to read splice point for [live/camera]
```

## Short description

SCTE-35 splice point cannot be read.

## Trigger condition

The SCTE-35 parser returns failure for the supplied section.

## Explanation

Live processing or ad-marker list processing returns failure without accepting this splice point.

## Possible causes

- The SCTE-35 parser returns failure for the supplied section.

## What to check

1. Check section framing, declared lengths, and splice-command data.

## Recommended action

Provide a complete supported SCTE-35 section.
