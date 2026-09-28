# Cannot allocate TLS Diffie-Hellman parameters

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-ff595235f60bc5af

## Log pattern

```text
DH_new() failed
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
DH_new() failed
```

## Short description

Cannot allocate TLS Diffie-Hellman parameters.

## Trigger condition

Allocation of the Diffie-Hellman parameter object fails while creating a certificate context.

## Explanation

The incomplete TLS context is freed and creation fails.

## Possible causes

- Allocation of the Diffie-Hellman parameter object fails while creating a certificate context.

## What to check

1. Check for memory allocation failures during certificate setup.
