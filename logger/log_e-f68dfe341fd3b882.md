# Cannot create TLS elliptic-curve key

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-f68dfe341fd3b882

## Log pattern

```text
unable to create ECDH curve name='%s'
```

## Parameters

- Argument 1 — Curve: Configured elliptic-curve name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unable to create ECDH curve name='prime256v1'
```

## Short description

Cannot create TLS elliptic-curve key.

## Trigger condition

The configured curve name is recognized but its temporary key object cannot be created.

## Explanation

The incomplete certificate context is freed and creation fails.

## Possible causes

- The configured curve name is recognized but its temporary key object cannot be created.

## What to check

1. Check the curve name and TLS allocation diagnostics.
