# URL verification digest update failed

Product: Nimble Streamer
Component: Encrypted playback URL validation
Logging function: log_e
Log level: error
Documentation ID: log_e-b48ccf5779265bba

## Log pattern

```text
EVP_DigestUpdate() failed
```

## Parameters

This pattern covers three separate digest updates: URL, initialization vector, and session key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
EVP_DigestUpdate() failed
```

## Short description

URL verification digest update failed.

## Trigger condition

During encrypted-link validation, updating the digest with decrypted URL bytes, initialization vector bytes, or session key bytes fails.

## Explanation

Validation returns failure and no URL is assigned.

## Possible causes

- The cryptographic library reports that one of the three verification digest updates failed.

## What to check

1. Check matching encrypted-link data and session key, and available cryptographic library operations.

## Recommended action

Resolve the failed decryption or digest operation before accepting the link.
