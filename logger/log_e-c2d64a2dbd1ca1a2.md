# KeyOS response has no current key-server settings

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-c2d64a2dbd1ca1a2

## Log pattern

```text
key server not found processing KeyOS_GetKeysRequest response keys_for='%s'
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
key server not found processing KeyOS_GetKeysRequest response keys_for='live/camera'
```

## Short description

KeyOS response has no current key-server settings.

## Trigger condition

A KeyOS response reaches processing after its key-server settings have disappeared.

## Explanation

Certificate validation and key decoding are not attempted.

## Possible causes

- A KeyOS response reaches processing after its key-server settings have disappeared.

## What to check

1. Check DRM settings changes during the KeyOS request.

## Recommended action

Check DRM settings changes during the KeyOS request.
