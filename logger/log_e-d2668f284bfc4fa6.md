# HLS encryption key decoded as empty

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-d2668f284bfc4fa6

## Log pattern

```text
non-empty key must be provided for 'hls_aes128' protection type
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
non-empty key must be provided for 'hls_aes128' protection type
```

## Short description

HLS encryption key decoded as empty.

## Trigger condition

Parsing the configured static key leaves all sixteen key bytes zero.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- Parsing the configured static key leaves all sixteen key bytes zero.

## What to check

1. Provide a valid nonzero hexadecimal AES-128 key.

## Recommended action

Provide a valid nonzero hexadecimal AES-128 key.
