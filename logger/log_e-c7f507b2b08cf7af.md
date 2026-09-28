# Publisher authorization stopped after an unknown exception

Product: Nimble Streamer
Component: Publisher authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-c7f507b2b08cf7af

## Log pattern

```text
unknown exception caught
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception caught
```

## Short description

Publisher authorization stopped after an unknown exception.

## Trigger condition

A nonstandard exception escapes the authorization request loop.

## Explanation

The thread stops after this message; no exception description or per-request default processing is provided by this catch.

## Possible causes

- An exception outside the standard exception handler escaped processing.

## What to check

1. Check the last authorization batch and prior diagnostics.

## Recommended action

Investigate the failure that stopped authorization processing.
