# Unsupported fragmented-MP4 initialization type

Product: Nimble Streamer
Component: Adaptive live delivery
Logging function: log_e
Log level: error
Documentation ID: log_e-0dbcdd7fd593e77c

## Log pattern

```text
unsupported fmp4 segment type: %d
```

## Parameters

- Argument 1 — segment\_type: Numeric segment-type value; supported video and audio initialization types are 0 and 1.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported fmp4 segment type: 2
```

## Short description

Unsupported fragmented-MP4 initialization type.

## Trigger condition

The adaptive stream resolves to an existing live stream, but the requested initialization segment type is neither video nor audio.

## Explanation

The lookup returns failure without retrieving an initialization segment.

## Possible causes

- An unsupported segment type reached initialization-segment lookup.

## What to check

1. Check the request path and segment-type selection.
