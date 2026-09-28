# Sigma DRM application identifier is missing

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-6af49db0ceda3609

## Log pattern

```text
App ID must be provided for Sigma DRM (sigma_drm_app_id)
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
App ID must be provided for Sigma DRM (sigma_drm_app_id)
```

## Short description

Sigma DRM application identifier is missing.

## Trigger condition

The Sigma DRM application identifier is empty.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- The Sigma DRM application identifier is empty.

## What to check

1. Provide the Sigma DRM application identifier.

## Recommended action

Provide the Sigma DRM application identifier.
