# CENC VOD audio chunk protection failed

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-25efb8fb2bf9bf17

## Log pattern

```text
failed encrypt cenc VOD audio fmp4 segment for [%s/%s]
```

## Parameters

- Argument 1 — Application: Text naming the VOD application.
- Argument 2 — Stream: Text naming the protected media stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed encrypt cenc VOD audio fmp4 segment for [vod/movie.mp4]
```

## Short description

CENC VOD audio chunk protection failed.

## Trigger condition

A generated HLS fragmented-MP4 audio chunk cannot be converted using its CENC protection parameters and the DRM helper returns no buffer.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- A generated HLS fragmented-MP4 audio chunk cannot be converted using its CENC protection parameters and the DRM helper returns no buffer.

## What to check

1. Inspect the preceding DRM segment-parameter and MP4 conversion errors for this stream; video also requires its prepared initialization segment.

## Recommended action

Inspect the preceding DRM segment-parameter and MP4 conversion errors for this stream; video also requires its prepared initialization segment.
