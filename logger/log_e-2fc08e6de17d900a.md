# CloudDRM login is missing

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-2fc08e6de17d900a

## Log pattern

```text
Login must be provided for Cloud DRM (cloud_drm_login)
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Login must be provided for Cloud DRM (cloud_drm_login)
```

## Short description

CloudDRM login is missing.

## Trigger condition

The CloudDRM login name is empty.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- The CloudDRM login name is empty.

## What to check

1. Provide the CloudDRM login name.

## Recommended action

Provide the CloudDRM login name.
