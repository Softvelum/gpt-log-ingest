# CENC video initialization segment preparation failed

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-eb15a80ccb5a2c7d

## Log pattern

```text
failed to encrypt cenc VOD video init fmp4 segment for [%s/%s]
```

## Parameters

- Argument 1 — Application: Text naming the VOD application.
- Argument 2 — Stream: Text naming the protected media stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to encrypt cenc VOD video init fmp4 segment for [vod/movie.mp4]
```

## Short description

CENC video initialization segment preparation failed.

## Trigger condition

A generated HLS fragmented-MP4 video initialization segment cannot be converted to its protected form and the DRM helper returns no buffer.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- A generated HLS fragmented-MP4 video initialization segment cannot be converted to its protected form and the DRM helper returns no buffer.

## What to check

1. Check the preceding DRM initialization-parameter and MP4 conversion errors, including whether protection parameters are available for this application and stream.

## Recommended action

Check the preceding DRM initialization-parameter and MP4 conversion errors, including whether protection parameters are available for this application and stream.
