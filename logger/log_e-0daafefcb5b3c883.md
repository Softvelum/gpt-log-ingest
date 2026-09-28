# ExpressPlay key request could not be submitted

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-0daafefcb5b3c883

## Log pattern

```text
[drm] failed to request key for '%s' from ExpressPlay url='%s' (retry index: %hu)
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.
- Argument 2 — Configured key-server URL: Text giving the configured key-server base URL; a request may add provider-specific path or query data.
- Argument 3 — Retry index: Unsigned 16-bit retry number; zero means the initial request.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] failed to request key for 'live/camera' from ExpressPlay url='https://keys.example/key' (retry index: 0)
```

## Short description

ExpressPlay key request could not be submitted.

## Trigger condition

The HTTP processor refuses the ExpressPlay key POST request.

## Explanation

No pending request is recorded for this attempt.

## Possible causes

- The HTTP processor refuses the ExpressPlay key POST request.

## What to check

1. Check the ExpressPlay endpoint and accompanying HTTP request-processor errors.

## Recommended action

Check the ExpressPlay endpoint and accompanying HTTP request-processor errors.
