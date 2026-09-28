# URL decryption finalization failed

Product: Nimble Streamer
Component: Encrypted playback URL validation
Logging function: log_e
Log level: error
Documentation ID: log_e-3bd36d77e9993cbe

## Log pattern

```text
EVP_DecryptFinal_ex() failed
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
EVP_DecryptFinal_ex() failed
```

## Short description

URL decryption finalization failed.

## Trigger condition

During encrypted-link validation, finalizing ciphertext decryption fails.

## Explanation

Validation returns failure and no URL is assigned.

## Possible causes

- The cryptographic library reports that the encrypted data cannot be finalized with the selected key and vector.

## What to check

1. Check matching encrypted-link data and session key, and available cryptographic library operations.

## Recommended action

Resolve the failed decryption or digest operation before accepting the link.
