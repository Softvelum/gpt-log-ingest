# TLS cipher list rejected

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-c0623ac7a6bd0abc

## Log pattern

```text
failed to set SSL cipher list s='%s' e='%s'
```

## Parameters

- Argument 1 — Cipher list: Configured TLS cipher-list expression.
- Argument 2 — Error: TLS library error description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set SSL cipher list s='INVALID' e='no cipher match'
```

## Short description

TLS cipher list rejected.

## Trigger condition

The TLS library cannot apply the configured cipher-list expression.

## Explanation

Certificate-context creation fails and the new context is discarded.

## Possible causes

- The TLS library cannot apply the configured cipher-list expression.

## What to check

1. Check the cipher-list expression and library error text.

## Recommended action

Use a cipher-list expression accepted by the installed TLS library.
