# DASH VOD segment protection failed

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-19e6cc38ce8ee751

## Log pattern

```text
failed encrypt VOD dash segment for [%s/%s]
```

## Parameters

- Argument 1 — Application: Text naming the VOD application.
- Argument 2 — Stream: Text naming the protected media stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed encrypt VOD dash segment for [vod/movie.mp4]
```

## Short description

DASH VOD segment protection failed.

## Trigger condition

The CENC protection helper returns no buffer for a generated DASH audio/video media or initialization segment.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- The CENC protection helper returns no buffer for a generated DASH audio/video media or initialization segment.

## What to check

1. Inspect preceding DRM initialization/segment parameter and MP4 conversion errors for this protected stream.

## Recommended action

Inspect preceding DRM initialization/segment parameter and MP4 conversion errors for this protected stream.
