# UDP sender cannot enable nonblocking mode

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_e
Log level: error
Documentation ID: log_e-b875613a40adc7ca

## Log pattern

```text
set non blocking failed.s=%d, (errno=%d)
```

## Parameters

- Argument 1 — Socket: Numeric socket descriptor.
- Argument 2 — Error: Operating-system socket error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
set non blocking failed.s=42, (errno=9)
```

## Short description

UDP sender cannot enable nonblocking mode.

## Trigger condition

The regular UDP sender socket rejects nonblocking setup.

## Explanation

Connection setup returns failure while the sender remains in shutdown state.

## Possible causes

- The regular UDP sender socket rejects nonblocking setup.

## What to check

1. Check the socket error and descriptor.

## Recommended action

Resolve the socket setup failure.
