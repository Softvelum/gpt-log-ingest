# CPIX content-key integrity check failed

Product: Nimble Streamer
Component: KeyOS DRM
Logging function: log_e
Log level: error
Documentation ID: log_e-7f5ac278dd3a62ca

## Log pattern

```text
failed to validate content key HMAC for CPIX response
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to validate content key HMAC for CPIX response
```

## Short description

CPIX content-key integrity check failed.

## Trigger condition

Required MAC, encrypted MAC key, encrypted content key, or document key is missing; alternatively RSA MAC-key decryption or HMAC computation/comparison fails.

## Explanation

The KeyOS response is rejected before content-key decryption.

## Possible causes

- Required MAC, encrypted MAC key, encrypted content key, or document key is missing; alternatively RSA MAC-key decryption or HMAC computation/comparison fails.

## What to check

1. Check the required CPIX key-delivery fields and that the response matches the configured partner RSA key.

## Recommended action

Check the required CPIX key-delivery fields and that the response matches the configured partner RSA key.
