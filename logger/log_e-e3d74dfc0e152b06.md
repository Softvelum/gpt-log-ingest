# Panel destination connection failed

Product: Nimble Streamer
Component: Panel synchronization and proxy transport
Logging function: log_e
Log level: error
Documentation ID: log_e-e3d74dfc0e152b06

## Log pattern

```text
failed to connect to %s (%s:%d) (errno=%d)
```

## Parameters

- Argument 1 — Host: Destination host or proxy host, text.
- Argument 2 — Address: Selected resolved IPv4 endpoint address, text.
- Argument 3 — Port: TCP destination port number, integer.
- Argument 4 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect to panel.example (192.0.2.10:443) (errno=5)
```

## Short description

Panel destination connection failed.

## Trigger condition

Connecting the TCP socket to one resolved destination or proxy address fails while silent mode is disabled.

## Explanation

The socket is closed and the next resolved address is tried.

## Possible causes

- The operating system rejected the TCP connection.

## What to check

1. Check the target or proxy address, port, and socket error.

## Recommended action

Restore connectivity to the reported endpoint.
