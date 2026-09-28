# SCTE-35 transport buffer cannot grow

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-5f211dd86be35a1c

## Log pattern

```text
[SCTE35] cannot allocate buffer message body for [%s/%s], used_size=%u, frame_size=%u
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.
- Argument 3 — Used size: Existing output buffer byte count.
- Argument 4 — Section size: Parsed SCTE-35 section size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] cannot allocate buffer message body for [live/camera], used_size=1048576, frame_size=35
```

## Short description

SCTE-35 transport buffer cannot grow.

## Trigger condition

The MPEG-TS output cannot reserve one additional 188-byte packet.

## Explanation

This transport-packaging call fails after raw forwarding and applicable marker queuing may already have occurred.

## Possible causes

- The MPEG-TS output cannot reserve one additional 188-byte packet.

## What to check

1. Check current transport-buffer use and the accepted section size.

## Recommended action

Resolve the transport-buffer capacity problem.
