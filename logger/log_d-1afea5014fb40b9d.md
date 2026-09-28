# EzDRM CPIX V2 protection selected

Product: Nimble Streamer
Component: DRM key management
Logging function: log_d
Log level: debug
Documentation ID: log_d-1afea5014fb40b9d

## Log pattern

```text
[drm] app='%s' stream='%s' is protected with EzDRM (CPIX V2)
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] app='live' stream='camera' is protected with EzDRM (CPIX V2)
```

## Short description

EzDRM CPIX V2 protection selected.

## Trigger condition

The application uses EzDRM CPIX V2 and stream registration includes MPEG-DASH or fragmented-MP4 HLS.

## Explanation

Key registration continues; this message alone does not confirm that usable keys have arrived.

## Possible causes

- The application uses EzDRM CPIX V2 and stream registration includes MPEG-DASH or fragmented-MP4 HLS.
