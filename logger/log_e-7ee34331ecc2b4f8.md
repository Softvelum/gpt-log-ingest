# Timeout setting is not positive

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-7ee34331ecc2b4f8

## Log pattern

```text
invalid timeout value='%s'
```

## Parameters

- Argument 1 — Timeout: Original timeout text; s/S means seconds, m/M minutes, otherwise hours.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid timeout value='0s'
```

## Short description

Timeout setting is not positive.

## Trigger condition

The numeric prefix of a timeout string converts to zero or less.

## Explanation

Timeout conversion fails and the caller rejects the setting. Accepted positive values use seconds for s/S, minutes for m/M, and hours otherwise.

## Possible causes

- The timeout is zero, negative, or has no positive numeric prefix.

## What to check

1. Check the original timeout text and its suffix.

## Recommended action

Provide a positive timeout with the intended unit suffix.
