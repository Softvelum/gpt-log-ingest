# Listener address could not be bound

Product: Nimble Streamer
Component: Socket communication
Logging function: log_e
Log level: error
Documentation ID: log_e-c0acfa15cf5435e9

## Log pattern

```text
failed to bind to %s:%d s=%d errno=%d
```

## Parameters

- Argument 1 — Host: Requested host or address text.
- Argument 2 — Port: TCP port number, integer.
- Argument 3 — Socket: Socket descriptor, integer; no unit.
- Argument 4 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

Both IPv4 and IPv6 listener setup use this format.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to bind to 192.0.2.10:1935 s=12 errno=5
```

## Short description

Listener address could not be bound.

## Trigger condition

Binding the newly opened IPv4 or IPv6 TCP listener to the requested address and port fails.

## Explanation

The socket is closed and listener setup returns failure.

## Possible causes

- The operating system rejected the requested local address and port binding.

## What to check

1. Check the local address, port, and reported socket error.

## Recommended action

Resolve the bind failure before starting the listener.
