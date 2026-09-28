# Advertising validation TLS write failed

Product: Nimble Streamer
Component: Advertising license validation
Logging function: log_e
Log level: error
Documentation ID: log_e-4f2148ddfc73bf21

## Log pattern

```text
Advertising validation procedure failed. Cannot send SSL request
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Advertising validation procedure failed. Cannot send SSL request
```

## Short description

Advertising validation TLS write failed.

## Trigger condition

The license-validation request could not be sent over TLS.

## Explanation

This license-validation attempt did not complete. Check the license-service connection and the associated transport diagnostics.

## Possible causes

- The license-validation request could not be sent over TLS.

## What to check

1. Check the accompanying TLS write error and license-server connectivity.

## Recommended action

Check the accompanying TLS write error and license-server connectivity.
