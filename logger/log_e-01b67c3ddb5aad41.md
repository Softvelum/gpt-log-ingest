# Required Quick Sync API version is unavailable

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-01b67c3ddb5aad41

## Log pattern

```text
failed to find quick sync >= %d.%d
```

## Parameters

- Argument 1 — Major version: Minimum requested media API major version, integer.
- Argument 2 — Minor version: Minimum requested media API minor version, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to find quick sync >= 1.17
```

## Short description

Required Quick Sync API version is unavailable.

## Trigger condition

Legacy automatic implementation selection returns the unsupported status for the requested API version.

## Explanation

Media session creation fails.

## Possible causes

- No selected implementation supports the requested minimum API version.

## What to check

1. Compare the reported minimum API version with the installed Intel media runtime and device support.

## Recommended action

Install a compatible media runtime or select a supported transcoding backend.
