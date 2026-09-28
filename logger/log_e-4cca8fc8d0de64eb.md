# URL verification digest initialization failed

Product: Nimble Streamer
Component: Encrypted playback URL validation
Logging function: log_e
Log level: error
Documentation ID: log_e-4cca8fc8d0de64eb

## Log pattern

```text
EVP_DigestInit_ex() failed
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
EVP_DigestInit_ex() failed
```

## Short description

URL verification digest initialization failed.

## Trigger condition

During encrypted-link validation, initializing the MD5 digest fails.

## Explanation

Validation returns failure and no URL is assigned.

## Possible causes

- The cryptographic library reports that the digest operation could not be initialized.

## What to check

1. Check matching encrypted-link data and session key, and available cryptographic library operations.

## Recommended action

Resolve the failed decryption or digest operation before accepting the link.
