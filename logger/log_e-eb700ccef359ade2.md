# Disconnect detection could not query pending bytes

Product: Nimble Streamer
Component: Socket event monitoring
Logging function: log_e
Log level: error
Documentation ID: log_e-eb700ccef359ade2

## Log pattern

```text
ioctlsocket(s=%d, FIONREAD, bytes_available) error=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ioctlsocket(s=12, FIONREAD, bytes_available) error=5
```

## Short description

Disconnect detection could not query pending bytes.

## Trigger condition

On Windows, a socket monitored only for disconnect becomes readable, but querying its pending byte count fails.

## Explanation

This pass cannot decide whether zero pending bytes indicate closure.

## Possible causes

- The Windows socket byte-count query failed.

## What to check

1. Check the socket descriptor and Windows socket error.

## Recommended action

Investigate the reported socket query error.

## Related log messages

- [Disconnect detection read the pending byte count](log_d-cafeec83723567f5.md)
