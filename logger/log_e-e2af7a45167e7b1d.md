# Axinom management key is missing

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-e2af7a45167e7b1d

## Log pattern

```text
Management Key must be provided for Axinom DRM (axinom_management_key)
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Management Key must be provided for Axinom DRM (axinom_management_key)
```

## Short description

Axinom management key is missing.

## Trigger condition

The Axinom management key is empty.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- The Axinom management key is empty.

## What to check

1. Provide the Axinom management key.

## Recommended action

Provide the Axinom management key.
