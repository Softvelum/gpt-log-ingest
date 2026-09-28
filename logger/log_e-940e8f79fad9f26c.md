# HLS chunk encryption preparation fails

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-940e8f79fad9f26c

## Log pattern

```text
hls chunk encryption failed for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
hls chunk encryption failed for [live/camera]
```

## Short description

HLS chunk encryption preparation fails.

## Trigger condition

Preparing the chunk for HLS encryption returns failure.

## Explanation

This chunk is not added to the HLS output. Processing continues, so this message alone does not establish that the stream stopped.

## Possible causes

- Preparing the chunk for HLS encryption returns failure.

## What to check

1. Check preceding DRM/key retrieval and encryption-buffer errors for this stream.

## Recommended action

Resolve the reported key or encryption preparation failure.
