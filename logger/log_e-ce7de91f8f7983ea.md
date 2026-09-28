# FairPlay key signaling or IV is missing

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-ce7de91f8f7983ea

## Log pattern

```text
[drm] FairPlay protection is not available for '%s/%s'
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] FairPlay protection is not available for 'live/camera'
```

## Short description

FairPlay key signaling or IV is missing.

## Trigger condition

HLS initialization or segment preparation finds an empty HLS key tag or an IV whose length is not 16 bytes.

## Explanation

The encryption parameter request fails.

## Possible causes

- HLS initialization or segment preparation finds an empty HLS key tag or an IV whose length is not 16 bytes.

## What to check

1. Check that the provider returns FairPlay HLS signaling and a 16-byte IV.

## Recommended action

Check that the provider returns FairPlay HLS signaling and a 16-byte IV.
