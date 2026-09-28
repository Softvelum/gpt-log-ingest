# DRM parameter is outside a settings group

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-da8fc937dad4f0bb

## Log pattern

```text
unexpected parameter out of drm settings group %s:%u '%s'
```

## Parameters

- Argument 1 — Settings file: Text naming the DRM settings file.
- Argument 2 — Settings line: Unsigned line number within the settings file.
- Argument 3 — Setting text: Text containing the rejected settings line.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unexpected parameter out of drm settings group /etc/nimble/drm.conf:12 'unsupported=value'
```

## Short description

DRM parameter is outside a settings group.

## Trigger condition

A nonempty non-comment line appears outside a recognized DRM group.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- A nonempty non-comment line appears outside a recognized DRM group.

## What to check

1. Place the parameter inside a DRM settings group.

## Recommended action

Place the parameter inside a DRM settings group.
