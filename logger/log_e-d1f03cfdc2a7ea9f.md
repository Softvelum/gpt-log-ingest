# HLS MPEG-TS part encryption preparation fails

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-d1f03cfdc2a7ea9f

## Log pattern

```text
hls mpegts part encryption failed for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
hls mpegts part encryption failed for [live/camera]
```

## Short description

HLS MPEG-TS part encryption preparation fails.

## Trigger condition

Preparing the MPEG-TS part for HLS encryption returns failure.

## Explanation

This MPEG-TS part is not added to the HLS output. Processing continues, so this message alone does not establish that the stream stopped.

## Possible causes

- Preparing the MPEG-TS part for HLS encryption returns failure.

## What to check

1. Check preceding DRM/key retrieval and encryption-buffer errors for this stream.

## Recommended action

Resolve the reported key or encryption preparation failure.
