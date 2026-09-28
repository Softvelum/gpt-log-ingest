# DRMToday login could not be submitted

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-c7885b4afcec9948

## Log pattern

```text
[drm] failed to request Login to receive key for '%s' from DRMToday url='%s' (retry index: %hu)
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.
- Argument 2 — Configured key-server URL: Text giving the key-server URL, not the authentication URL used by this POST.
- Argument 3 — Retry index: Unsigned 16-bit retry number; zero means the initial request.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] failed to request Login to receive key for 'live/camera' from DRMToday url='https://keys.example/key' (retry index: 0)
```

## Short description

DRMToday login could not be submitted.

## Trigger condition

The HTTP processor refuses the DRMToday login POST.

## Explanation

The application key set is erased and no pending login request is recorded.

## Possible causes

- The HTTP processor refuses the DRMToday login POST.

## What to check

1. Check the DRMToday authentication endpoint and request-processor errors.

## Recommended action

Check the DRMToday authentication endpoint and request-processor errors.
