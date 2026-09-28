# SCTE-35 section exceeds the one-packet transport limit

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-edb6ea7c02bc7eac

## Log pattern

```text
[SCTE35] max splice info size exceeded for [%s/%s], frame_size=%u
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.
- Argument 3 — Section size: Complete parsed section size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] max splice info size exceeded for [live/camera], frame_size=184
```

## Short description

SCTE-35 section exceeds the one-packet transport limit.

## Trigger condition

The parsed splice section exceeds the supported 183-byte payload limit while MPEG-TS over HTTP output is enabled.

## Explanation

Transport packaging fails after raw forwarding and applicable HLS/DASH marker queuing may already have occurred.

## Possible causes

- The parsed splice section exceeds the supported 183-byte payload limit while MPEG-TS over HTTP output is enabled.

## What to check

1. Check the full splice-section byte count.

## Recommended action

Reduce SCTE-35 section size to the supported transport limit.
