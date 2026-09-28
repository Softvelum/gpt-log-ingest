# Outbound connection attempt was rejected

Product: Nimble Streamer
Component: Socket communication
Logging function: log_e
Log level: error
Documentation ID: log_e-e82de05c06440eed

## Log pattern

```text
failed to connect s=%d '%s:%d' errno=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Host: Requested host or address text.
- Argument 3 — Port: TCP port number, integer.
- Argument 4 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect s=12 '192.0.2.10:1935' errno=5
```

## Short description

Outbound connection attempt was rejected.

## Trigger condition

The connect result does not match the expected nonblocking connection-in-progress case.

## Explanation

This socket is closed; any remaining resolved addresses are tried. An immediate successful connect also enters this branch in this implementation.

## Possible causes

- The connect call returned another result; the displayed error was read after that call and may be stale for immediate success.

## What to check

1. Check the target address and port and the actual connect result in surrounding diagnostics.

## Recommended action

Investigate why the connection did not enter the expected asynchronous state.

## Related log messages

- [TCP connection started or UDP input bound](log_v-a663a17157839920.md)
