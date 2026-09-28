# Listener could not enter nonblocking mode

Product: Nimble Streamer
Component: Socket communication
Logging function: log_e
Log level: error
Documentation ID: log_e-fdd2f6dbd4a1eb4e

## Log pattern

```text
unable to set non-blocking socket s=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unable to set non-blocking socket s=12
```

## Short description

Listener could not enter nonblocking mode.

## Trigger condition

Setting a newly bound listener to nonblocking mode fails.

## Explanation

The listener is closed and setup returns failure.

## Possible causes

- The platform socket-mode operation failed.

## What to check

1. Check the preceding socket-mode diagnostic.

## Recommended action

Resolve the socket-mode failure.

## Related log messages

- [Windows socket mode change failed](log_e-5b7d38a8eb06ce7e.md)
- [Nonblocking socket flag could not be set](log_e-7a0d8b96c99d483f.md)
- [Socket flags could not be read](log_e-cd39644ecdf782b6.md)
