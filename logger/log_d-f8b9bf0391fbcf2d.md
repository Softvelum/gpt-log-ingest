# Unreferenced DRM key removed

Product: Nimble Streamer
Component: DRM key management
Logging function: log_d
Log level: debug
Documentation ID: log_d-f8b9bf0391fbcf2d

## Log pattern

```text
removed not required drm key='%s' for stream='%s'
```

## Parameters

- Argument 1 — Key-server URL: Text giving the configured or requested key-server URL.
- Argument 2 — Key owner: Text identifying the application or application/stream whose keys are managed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
removed not required drm key='https://keys.example/key' for stream='live/camera'
```

## Short description

Unreferenced DRM key removed.

## Trigger condition

An application key set is removed and a key has no remaining references.

## Explanation

The key is removed from the server-URL lookup.

## Possible causes

- An application key set is removed and a key has no remaining references.
