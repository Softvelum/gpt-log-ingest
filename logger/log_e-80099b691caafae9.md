# Encrypted URL hash has the wrong decoded size

Product: Nimble Streamer
Component: Encrypted playback URL validation
Logging function: log_e
Log level: error
Documentation ID: log_e-80099b691caafae9

## Log pattern

```text
BIO_read failed read=%u expected=16
```

## Parameters

- Argument 1 — Decoded bytes: Decoded hash length stored as an unsigned integer; a negative decoder result appears as a large unsigned value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BIO_read failed read=8 expected=16
```

## Short description

Encrypted URL hash has the wrong decoded size.

## Trigger condition

Base64 decoding the supplied hash does not produce exactly 16 bytes.

## Explanation

URL validation returns failure without assigning a decrypted URL.

## Possible causes

- The hash is malformed or has the wrong length.

## What to check

1. Check that all encrypted-link fields are complete and correctly encoded.

## Recommended action

Regenerate the complete encrypted link with matching fields.
