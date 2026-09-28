# Socket flags could not be read

Product: Nimble Streamer
Component: Socket communication
Logging function: log_e
Log level: error
Documentation ID: log_e-cd39644ecdf782b6

## Log pattern

```text
fcntl F_GETFL failed errno=%d for s=%d
```

## Parameters

- Argument 1 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.
- Argument 2 — Socket: Socket descriptor, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
fcntl F_GETFL failed errno=5 for s=12
```

## Short description

Socket flags could not be read.

## Trigger condition

On non-Windows platforms, reading socket status flags fails before enabling nonblocking mode.

## Explanation

Nonblocking setup returns failure.

## Possible causes

- The operating system rejected the socket-mode operation.

## What to check

1. Check the socket descriptor and reported error.

## Recommended action

Resolve the socket-mode error before using the socket asynchronously.
