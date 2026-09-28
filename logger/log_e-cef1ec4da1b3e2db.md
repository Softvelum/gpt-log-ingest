# KeyOS user private key cannot be read

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-cef1ec4da1b3e2db

## Log pattern

```text
failed to load KeyOS user key from '%s'
```

## Parameters

- Argument 1 — Credential file: Text naming the KeyOS user private key file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load KeyOS user key from '/data/keyos.pem'
```

## Short description

KeyOS user private key cannot be read.

## Trigger condition

Reading the configured KeyOS user private key file fails.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- Reading the configured KeyOS user private key file fails.

## What to check

1. Correct the KeyOS user private key path and restore read access.

## Recommended action

Correct the KeyOS user private key path and restore read access.
