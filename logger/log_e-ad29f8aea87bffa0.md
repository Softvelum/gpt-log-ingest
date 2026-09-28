# Unknown TLS elliptic curve

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-ad29f8aea87bffa0

## Log pattern

```text
unknown ECDH curve name='%s'
```

## Parameters

- Argument 1 — Curve: Configured elliptic-curve name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown ECDH curve name='unknown_curve'
```

## Short description

Unknown TLS elliptic curve.

## Trigger condition

The configured elliptic-curve name has no recognized library identifier.

## Explanation

Certificate-context creation fails. This check applies when elliptic-curve support is available in the TLS build.

## Possible causes

- The configured elliptic-curve name has no recognized library identifier.

## What to check

1. Check the configured elliptic-curve name.

## Recommended action

Choose a curve name recognized by the TLS library.
