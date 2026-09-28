# ExpressPlay protection selected

Product: Nimble Streamer
Component: DRM key management
Logging function: log_d
Log level: debug
Documentation ID: log_d-d6baee0390a97cbf

## Log pattern

```text
[drm] app='%s' stream='%s' is protected with ExpressPlay
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] app='live' stream='camera' is protected with ExpressPlay
```

## Short description

ExpressPlay protection selected.

## Trigger condition

The application uses ExpressPlay and stream registration includes MPEG-DASH or fragmented-MP4 HLS.

## Explanation

Key registration continues; this message alone does not confirm that usable keys have arrived.

## Possible causes

- The application uses ExpressPlay and stream registration includes MPEG-DASH or fragmented-MP4 HLS.
