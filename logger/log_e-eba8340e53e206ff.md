# KeyOS partner certificate cannot be read

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-eba8340e53e206ff

## Log pattern

```text
failed to load KeyOS partner certificate from '%s'
```

## Parameters

- Argument 1 — Credential file: Text naming the KeyOS partner certificate file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load KeyOS partner certificate from '/data/keyos.pem'
```

## Short description

KeyOS partner certificate cannot be read.

## Trigger condition

Reading the configured KeyOS partner certificate file fails.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- Reading the configured KeyOS partner certificate file fails.

## What to check

1. Correct the KeyOS partner certificate path and restore read access.

## Recommended action

Correct the KeyOS partner certificate path and restore read access.
