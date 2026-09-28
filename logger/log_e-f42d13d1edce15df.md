# DRMToday ticket request could not be submitted

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-f42d13d1edce15df

## Log pattern

```text
[drm] failed to request CAS ticket to receive key for '%s' from DRMToday url='%s' (retry index: %hu)
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.
- Argument 2 — Login URL: Text giving the ticket submission URL returned by login.
- Argument 3 — Retry index: Unsigned 16-bit retry number; zero means the initial request.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] failed to request CAS ticket to receive key for 'live/camera' from DRMToday url='https://auth.example/ticket' (retry index: 0)
```

## Short description

DRMToday ticket request could not be submitted.

## Trigger condition

The HTTP processor refuses the CAS-ticket POST to the login URL.

## Explanation

The application key set is erased and no pending ticket request is recorded.

## Possible causes

- The HTTP processor refuses the CAS-ticket POST to the login URL.

## What to check

1. Check the URL returned by login and the accompanying HTTP request-processor errors.

## Recommended action

Check the URL returned by login and the accompanying HTTP request-processor errors.
