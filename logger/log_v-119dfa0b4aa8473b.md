# CPIX DRM key installed

Product: Nimble Streamer
Component: DRM key management
Logging function: log_v
Log level: verbose
Documentation ID: log_v-119dfa0b4aa8473b

## Log pattern

```text
[drm] added DRM key for '%s' from %s
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.
- Argument 2 — Provider: Text naming the DRM provider.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] added DRM key for 'live/camera' from Widevine
```

## Short description

CPIX DRM key installed.

## Trigger condition

A parsed CPIX response contains one key and matches one stored slot.

## Explanation

Key material, IV, and supported HLS/DASH signaling are stored; waiting consumers are notified of success.

## Possible causes

- A parsed CPIX response contains one key and matches one stored slot.
