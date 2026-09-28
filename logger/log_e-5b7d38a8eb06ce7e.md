# Windows socket mode change failed

Product: Nimble Streamer
Component: Socket communication
Logging function: log_e
Log level: error
Documentation ID: log_e-5b7d38a8eb06ce7e

## Log pattern

```text
ioctlsocket FIONBIO failed errno=%d for s=%d
```

## Parameters

- Argument 1 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.
- Argument 2 — Socket: Socket descriptor, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ioctlsocket FIONBIO failed errno=5 for s=12
```

## Short description

Windows socket mode change failed.

## Trigger condition

The Windows request to enable nonblocking socket mode fails.

## Explanation

Nonblocking setup returns failure.

## Possible causes

- The operating system rejected the socket-mode operation.

## What to check

1. Check the socket descriptor and reported error.

## Recommended action

Resolve the socket-mode error before using the socket asynchronously.
