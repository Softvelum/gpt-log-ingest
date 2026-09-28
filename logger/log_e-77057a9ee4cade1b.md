# DRM settings could not be reloaded

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-77057a9ee4cade1b

## Log pattern

```text
failed to load drm.conf to apply DRM settings
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load drm.conf to apply DRM settings
```

## Short description

DRM settings could not be reloaded.

## Trigger condition

Reading or parsing DRM settings fails during an explicit reload.

## Explanation

The requested settings are not applied and reload reports failure.

## Possible causes

- Reading or parsing DRM settings fails during an explicit reload.

## What to check

1. Check the preceding settings-file or parameter validation error.

## Recommended action

Check the preceding settings-file or parameter validation error.
