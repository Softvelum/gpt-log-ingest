# Local socket address has an unsupported family

Product: Nimble Streamer
Component: Socket communication
Logging function: log_e
Log level: error
Documentation ID: log_e-d1a80394c8e4df0f

## Log pattern

```text
getsockname s=%d returned unexpected addr family=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Address family: Operating system address-family identifier, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
getsockname s=12 returned unexpected addr family=1
```

## Short description

Local socket address has an unsupported family.

## Trigger condition

The local-address query succeeds but returns neither IPv4 nor IPv6.

## Explanation

The address helper returns empty text.

## Possible causes

- The queried socket has an unsupported address family.

## What to check

1. Check the returned address-family identifier.

## Recommended action

Use an IPv4 or IPv6 socket.
