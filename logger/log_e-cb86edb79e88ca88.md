# HTTP client setup raised an unclassified exception

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_e
Log level: error
Documentation ID: log_e-cb86edb79e88ca88

## Log pattern

```text
unknown exception while creating client s=%d
```

## Parameters

- Argument 1 — Socket: Integer socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception while creating client s=12
```

## Short description

HTTP client setup raised an unclassified exception.

## Trigger condition

Accepted-client setup throws an exception outside the allocation-specific handler.

## Explanation

The partially created session or socket is released, and the descriptor is removed from the active map.

## Possible causes

- Session setup raised an unclassified exception.

## What to check

1. Inspect preceding setup diagnostics for this connection.
