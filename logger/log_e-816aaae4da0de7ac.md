# Management TLS accept initialization failed

Product: Nimble Streamer
Component: HTTP management API
Logging function: log_e
Log level: error
Documentation ID: log_e-816aaae4da0de7ac

## Log pattern

```text
failed to init ssl s=%d
```

## Parameters

- Argument 1 — socket: Accepted management socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to init ssl s=42
```

## Short description

Management TLS accept initialization failed.

## Trigger condition

A newly created client on a TLS listener cannot initialize its TLS accept state.

## Explanation

The client is deleted and the handler returns before registering it in the management client table.

## Possible causes

- TLS initialization for the accepted socket returned failure.

## What to check

1. Check preceding TLS context and certificate diagnostics.

## Recommended action

Correct the indicated TLS setup failure.
