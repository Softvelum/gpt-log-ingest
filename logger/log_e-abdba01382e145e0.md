# DRM parameter is unknown

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-abdba01382e145e0

## Log pattern

```text
unknown parameter at %s:%u '%s'
```

## Parameters

- Argument 1 — Settings file: Text naming the DRM settings file.
- Argument 2 — Settings line: Unsigned line number within the settings file.
- Argument 3 — Setting text: Text containing the rejected settings line.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown parameter at /etc/nimble/drm.conf:12 'unsupported=value'
```

## Short description

DRM parameter is unknown.

## Trigger condition

A parameter inside the DRM settings group is not recognized.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- A parameter inside the DRM settings group is not recognized.

## What to check

1. Correct or remove the unrecognized parameter.

## Recommended action

Correct or remove the unrecognized parameter.
