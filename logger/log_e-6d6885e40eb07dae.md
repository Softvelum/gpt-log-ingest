# Encrypted URL fields are missing

Product: Nimble Streamer
Component: Encrypted playback URL validation
Logging function: log_e
Log level: error
Documentation ID: log_e-6d6885e40eb07dae

## Log pattern

```text
enc_url_base64 size=%zu iv_base64 size=%zu hash_base64 size=%zu
```

## Parameters

- Argument 1 — Encrypted URL length: Base64 text length in bytes, unsigned size value.
- Argument 2 — Initialization vector length: Base64 text length in bytes, unsigned size value.
- Argument 3 — Hash length: Base64 text length in bytes, unsigned size value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
enc_url_base64 size=0 iv_base64 size=24 hash_base64 size=24
```

## Short description

Encrypted URL fields are missing.

## Trigger condition

The encrypted URL, initialization vector, or supplied hash text is empty.

## Explanation

URL validation returns failure without assigning a decrypted URL.

## Possible causes

- At least one required encrypted-link field is missing.

## What to check

1. Check that all encrypted-link fields are complete and correctly encoded.

## Recommended action

Regenerate the complete encrypted link with matching fields.
