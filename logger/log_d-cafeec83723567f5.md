# Disconnect detection read the pending byte count

Product: Nimble Streamer
Component: Socket event monitoring
Logging function: log_d
Log level: debug
Documentation ID: log_d-cafeec83723567f5

## Log pattern

```text
ioctlsocket(s=%d, FIONREAD, bytes_available=%lu)
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Pending bytes: Bytes available to read, unsigned long integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ioctlsocket(s=12, FIONREAD, bytes_available=0)
```

## Short description

Disconnect detection read the pending byte count.

## Trigger condition

On Windows, a readable socket monitored only for disconnect reports its pending byte count.

## Explanation

Zero bytes cause a disconnect event. A positive count removes disconnect-only monitoring because data is pending.

## Related log messages

- [Disconnect detection could not query pending bytes](log_e-eb700ccef359ade2.md)
