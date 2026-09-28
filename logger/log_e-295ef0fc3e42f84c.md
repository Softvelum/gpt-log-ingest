# DRMToday login request failed

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-295ef0fc3e42f84c

## Log pattern

```text
DRMToday failed to login request url='%s' %s
```

## Parameters

- Argument 1 — Key-server URL: Text giving the configured or requested key-server URL.
- Argument 2 — HTTP response details: Text containing the HTTP status and optional response body; empty when no response parser exists.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DRMToday failed to login request url='https://keys.example/key' status=503 body='unavailable'
```

## Short description

DRMToday login request failed.

## Trigger condition

The HTTP processor reports a failure for DRMToday login.

## Explanation

The relevant keys are marked failed and waiting consumers are notified.

## Possible causes

- The HTTP processor reports a failure for DRMToday login.

## What to check

1. Check the HTTP status and response details for the DRMToday login endpoint.

## Recommended action

Check the HTTP status and response details for the DRMToday login endpoint.
