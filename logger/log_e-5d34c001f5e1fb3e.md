# Encrypted URL hash text is too long

Product: Nimble Streamer
Component: Encrypted playback URL validation
Logging function: log_e
Log level: error
Documentation ID: log_e-5d34c001f5e1fb3e

## Log pattern

```text
hash_base64 length=%zu is too long
```

## Parameters

- Argument 1 — Hash length: Base64 hash text length in bytes, unsigned size value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
hash_base64 length=32 is too long
```

## Short description

Encrypted URL hash text is too long.

## Trigger condition

The encoded hash length fails the preliminary bound for a 16-byte digest.

## Explanation

URL validation returns failure without assigning a decrypted URL.

## Possible causes

- The supplied encoded hash exceeds the accepted bound.

## What to check

1. Check that all encrypted-link fields are complete and correctly encoded.

## Recommended action

Regenerate the complete encrypted link with matching fields.
