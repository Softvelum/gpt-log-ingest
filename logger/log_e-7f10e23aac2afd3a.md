# VCAS settings disappeared after asset creation

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-7f10e23aac2afd3a

## Log pattern

```text
key server not found after processing VCAS CreateAsset keys_for='%s'
```

## Parameters

- Argument 1 — Key owner: Text identifying the application or application/stream whose keys are managed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
key server not found after processing VCAS CreateAsset keys_for='live/camera'
```

## Short description

VCAS settings disappeared after asset creation.

## Trigger condition

The create-asset response succeeds but its application no longer has a key server.

## Explanation

No follow-on key-creation requests are sent.

## Possible causes

- The create-asset response succeeds but its application no longer has a key server.

## What to check

1. Check DRM settings changes while the asset request was in progress.

## Recommended action

Check DRM settings changes while the asset request was in progress.
