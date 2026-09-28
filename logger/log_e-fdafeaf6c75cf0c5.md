# Advertising validation response buffer allocation failed

Product: Nimble Streamer
Component: Advertising license validation
Logging function: log_e
Log level: error
Documentation ID: log_e-fdafeaf6c75cf0c5

## Log pattern

```text
Advertising validation procedure failed due to internal error
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Advertising validation procedure failed due to internal error
```

## Short description

Advertising validation response buffer allocation failed.

## Trigger condition

The license-validation request could not allocate its response buffer.

## Explanation

This license-validation attempt did not complete because its response buffer could not be allocated. Check process memory and resource availability.

## Possible causes

- The license-validation request could not allocate its response buffer.

## What to check

1. Check process memory pressure.

## Recommended action

Check process memory pressure.
