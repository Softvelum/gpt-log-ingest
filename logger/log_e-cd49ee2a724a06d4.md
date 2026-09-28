# Cannot set SRT encryption passphrase

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-cd49ee2a724a06d4

## Log pattern

```text
failed to set passphrase=%s, s=%d
```

## Parameters

- Argument 1 — Passphrase: Configured passphrase text.
- Argument 2 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set passphrase=example-only-passphrase, s=100
```

## Short description

Cannot set SRT encryption passphrase.

## Trigger condition

The key length was accepted but SRT rejects setting the configured passphrase.

## Explanation

Socket settings application returns failure. The message includes the configured passphrase text.

## Possible causes

- The key length was accepted but SRT rejects setting the configured passphrase.

## What to check

1. Check the configured passphrase and the socket setup state.
