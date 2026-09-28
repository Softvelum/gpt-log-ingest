# Low-latency audio HLS part cannot grow

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-d6fdef6523ebb036

## Log pattern

```text
[AM] cannot allocate hls part for [%s/%s], used_size=%u, frame_size=%u, ts=%llu
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.
- Argument 3 — Used size: Bytes already used in the affected output buffer.
- Argument 4 — Frame size: Media payload size in bytes, including any prepared transport header.
- Argument 5 — Timestamp: Audio timestamp in 90 kHz ticks.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[AM] cannot allocate hls part for [live/camera], used_size=1048576, frame_size=4096, ts=90000
```

## Short description

Low-latency audio HLS part cannot grow.

## Trigger condition

Appending audio bytes to an initialized audio-only HLS part exceeds the available allocation.

## Explanation

The audio-part operation fails without appending this payload.

## Possible causes

- Appending audio bytes to an initialized audio-only HLS part exceeds the available allocation.

## What to check

1. Check the audio-part byte count, payload size, and allocation limits.

## Recommended action

Resolve the part-buffer capacity problem.
