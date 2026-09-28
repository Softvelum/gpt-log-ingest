# Chunked external response exceeds its buffer

Product: Nimble Streamer
Component: External HTTP requests
Logging function: log_e
Log level: error
Documentation ID: log_e-ba8a6c8b2f510a70

## Log pattern

```text
chunked data size=%lld exceeds buffer capacity=%lld url='%s'
```

## Parameters

- Argument 1 — body bytes: Current body bytes plus newly parsed data.
- Argument 2 — capacity: Body buffer capacity in bytes.
- Argument 3 — URL: Original external request URL, retained even when following a redirect.

Platform-sized byte counts are passed to signed 64-bit conversions.

## Log examples

No portable example is provided for the argument-type mismatch.

## Short description

Chunked external response exceeds its buffer.

## Trigger condition

Decoded chunk data would make the collected body larger than its buffer.

## Explanation

This check occurs both when headers first contain body data and during later chunk processing. The session is marked finished before copying the excess data.

## Possible causes

- The response body is larger than the request destination buffer.

## What to check

1. Compare the response size with the supplied buffer or request buffer settings.

## Recommended action

Provide sufficient response capacity or correct the returned body size.
