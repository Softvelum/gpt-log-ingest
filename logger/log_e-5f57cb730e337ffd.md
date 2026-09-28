# Additional TLS certificate has no server name

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-5f57cb730e337ffd

## Log pattern

```text
server_name shall not be empty for cert='%s'
```

## Parameters

- Argument 1 — Certificate: Configured certificate-chain file path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
server_name shall not be empty for cert='/data/certs/secondary.pem'
```

## Short description

Additional TLS certificate has no server name.

## Trigger condition

A nondefault certificate has no exact, wildcard, or regular-expression server-name match.

## Explanation

Context creation fails. Only the default certificate context may have an empty server-name list.

## Possible causes

- A nondefault certificate has no exact, wildcard, or regular-expression server-name match.

## What to check

1. Check hostname mappings for the indicated certificate.

## Recommended action

Assign the intended server name to this additional certificate.
