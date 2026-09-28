# Advertising validation request was only partly sent

Product: Nimble Streamer
Component: Advertising license validation
Logging function: log_e
Log level: error
Documentation ID: log_e-087666cab44f2ccc

## Log pattern

```text
Advertising validation procedure failed. SSL request send partially
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Advertising validation procedure failed. SSL request send partially
```

## Short description

Advertising validation request was only partly sent.

## Trigger condition

The license-validation TLS request was only partly sent.

## Explanation

This license-validation attempt did not complete. Check the license-service connection and the associated transport diagnostics.

## Possible causes

- The license-validation TLS request was only partly sent.

## What to check

1. Check the license-server connection and TLS write failures.

## Recommended action

Check the license-server connection and TLS write failures.
