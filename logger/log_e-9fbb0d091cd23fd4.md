# DRM response has no pending request

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-9fbb0d091cd23fd4

## Log pattern

```text
VCAS response id=%llu url='%s' was not expected
```

## Parameters

- Argument 1 — Request ID: Unsigned identifier of the completed request.
- Argument 2 — Key-server URL: Text giving the configured or requested key-server URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
VCAS response id=42 url='https://keys.example/key' was not expected
```

## Short description

DRM response has no pending request.

## Trigger condition

A success or failure callback carries a nonzero request ID absent from the pending DRM requests.

## Explanation

The callback is ignored. The wording mentions VCAS but the guard covers all managed DRM providers.

## Possible causes

- A success or failure callback carries a nonzero request ID absent from the pending DRM requests.

## What to check

1. Check whether settings changed or the same request completed more than once.

## Recommended action

Check whether settings changed or the same request completed more than once.
