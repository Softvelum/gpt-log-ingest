# UDP sender cannot bind its requested local endpoint

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_e
Log level: error
Documentation ID: log_e-cba3dae2a5d9a854

## Log pattern

```text
bind UDP socket to s=%d (errno=%d)
```

## Parameters

- Argument 1 — Socket: Numeric socket descriptor.
- Argument 2 — Error: Operating-system socket error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
bind UDP socket to s=42 (errno=98)
```

## Short description

UDP sender cannot bind its requested local endpoint.

## Trigger condition

An explicitly configured local address or port cannot be bound.

## Explanation

Both regular and constant-rate UDP setup log the error and continue, allowing later sends to use a system-selected local endpoint.

## Possible causes

- An explicitly configured local address or port cannot be bound.

## What to check

1. Check local endpoint availability, port conflicts, and socket error.

## Recommended action

Correct the local bind settings if that endpoint is required.
