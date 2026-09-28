# Advertising validation socket could not attach to TLS

Product: Nimble Streamer
Component: Advertising license validation
Logging function: log_e
Log level: error
Documentation ID: log_e-d973edce2f145b06

## Log pattern

```text
Advertising validation procedure failed. Cannot bind socket to SSL session
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Advertising validation procedure failed. Cannot bind socket to SSL session
```

## Short description

Advertising validation socket could not attach to TLS.

## Trigger condition

The license-validation connection could not be attached to its TLS session.

## Explanation

This license-validation attempt did not complete. Check the license-service connection and the associated transport diagnostics.

## Possible causes

- The license-validation connection could not be attached to its TLS session.

## What to check

1. Check socket and TLS initialization failures.

## Recommended action

Check socket and TLS initialization failures.
