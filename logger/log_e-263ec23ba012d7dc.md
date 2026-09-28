# KeyOS partner private key cannot be read

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-263ec23ba012d7dc

## Log pattern

```text
failed to load KeyOS partner key from '%s'
```

## Parameters

- Argument 1 — Credential file: Text naming the KeyOS partner private key file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load KeyOS partner key from '/data/keyos.pem'
```

## Short description

KeyOS partner private key cannot be read.

## Trigger condition

Reading the configured KeyOS partner private key file fails.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- Reading the configured KeyOS partner private key file fails.

## What to check

1. Correct the KeyOS partner private key path and restore read access.

## Recommended action

Correct the KeyOS partner private key path and restore read access.
