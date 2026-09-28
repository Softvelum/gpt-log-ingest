# DRM initialization vector cannot be parsed

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-ce2b921b7b57b4dd

## Log pattern

```text
invalid iv='%s' at %s:%u
```

## Parameters

- Argument 1 — IV text: Text containing the rejected initialization vector.
- Argument 2 — Settings file: Text naming the DRM settings file.
- Argument 3 — Settings line: Unsigned line number within the settings file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid iv='invalid' at /etc/nimble/drm.conf:12
```

## Short description

DRM initialization vector cannot be parsed.

## Trigger condition

Parsing the configured IV does not produce any hexadecimal value.

## Explanation

The invalid IV is ignored and settings parsing continues.

## Possible causes

- Parsing the configured IV does not produce any hexadecimal value.

## What to check

1. Supply a valid hexadecimal initialization vector.

## Recommended action

Supply a valid hexadecimal initialization vector.
