# CPIX validation error was permitted

Product: Nimble Streamer
Component: KeyOS DRM
Logging function: log_e
Log level: error
Documentation ID: log_e-bb86e82f275f0bd9

## Log pattern

```text
CPIX validation failed but skipped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
CPIX validation failed but skipped
```

## Short description

CPIX validation error was permitted.

## Trigger condition

Nonfatal validation mode encounters a canonicalization failure, missing or mismatched digest/signature fields, unreadable signing certificate, missing public key, or verification-context allocation failure.

## Explanation

That validation error is treated as success and later key integrity and decryption checks continue. A final cryptographic signature mismatch is still fatal.

## Possible causes

- Nonfatal validation mode encounters a canonicalization failure, missing or mismatched digest/signature fields, unreadable signing certificate, missing public key, or verification-context allocation failure.

## What to check

1. Correct the response or signing certificate and verify the intended response-validation setting.

## Recommended action

Correct the response or signing certificate and verify the intended response-validation setting.
