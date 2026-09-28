# Empty DRM key slot created

Product: Nimble Streamer
Component: DRM key management
Logging function: log_d
Log level: debug
Documentation ID: log_d-dc0a726fc2bec0c9

## Log pattern

```text
[drm] created empty key for '%s'
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[drm] created empty key for 'live/camera'
```

## Short description

Empty DRM key slot created.

## Trigger condition

A CPIX-based stream has no key set and a single empty key slot is allocated.

## Explanation

The corresponding provider request starts next; the slot has no usable key yet.

## Possible causes

- A CPIX-based stream has no key set and a single empty key slot is allocated.
