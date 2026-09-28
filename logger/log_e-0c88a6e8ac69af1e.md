# Local socket address is unavailable

Product: Nimble Streamer
Component: Socket communication
Logging function: log_e
Log level: error
Documentation ID: log_e-0c88a6e8ac69af1e

## Log pattern

```text
getsockname s=%d failed errno=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
getsockname s=12 failed errno=5
```

## Short description

Local socket address is unavailable.

## Trigger condition

Querying the local socket address fails.

## Explanation

The address helper returns empty text without assigning a port.

## Possible causes

- The operating system rejected the local-address query.

## What to check

1. Check the socket descriptor and error.

## Recommended action

Resolve the local-address query failure.
