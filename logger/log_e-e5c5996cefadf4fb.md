# CPIX response cannot be parsed

Product: Nimble Streamer
Component: KeyOS DRM
Logging function: log_e
Log level: error
Documentation ID: log_e-e5c5996cefadf4fb

## Log pattern

```text
failed to parse CPIX response
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse CPIX response
```

## Short description

CPIX response cannot be parsed.

## Trigger condition

The partner private key cannot supply an RSA key, the XML parser cannot be allocated, or the response is not parseable XML.

## Explanation

The KeyOS response is rejected.

## Possible causes

- The partner private key cannot supply an RSA key, the XML parser cannot be allocated, or the response is not parseable XML.

## What to check

1. Check the RSA partner key and returned CPIX XML; inspect resource errors if parser allocation failed.

## Recommended action

Check the RSA partner key and returned CPIX XML; inspect resource errors if parser allocation failed.
