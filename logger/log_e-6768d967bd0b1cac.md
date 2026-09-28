# Widevine key request could not be submitted

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-6768d967bd0b1cac

## Log pattern

```text
[drm] failed to request Widevine keys for '%s' cpi=%u/%u retry=%hu
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.
- Argument 2 — Crypto-period index: Unsigned key rotation period index.
- Argument 3 — Period count: Unsigned number of consecutive crypto periods.
- Argument 4 — Retry index: Unsigned 16-bit retry number; zero means the initial request.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] failed to request Widevine keys for 'live/camera' cpi=120/2 retry=0
```

## Short description

Widevine key request could not be submitted.

## Trigger condition

The HTTP processor refuses a prepared Widevine key POST.

## Explanation

No pending request is recorded for this attempt, although placeholder request times were already updated.

## Possible causes

- The HTTP processor refuses a prepared Widevine key POST.

## What to check

1. Check the Widevine endpoint and HTTP request-processor errors.

## Recommended action

Check the Widevine endpoint and HTTP request-processor errors.
