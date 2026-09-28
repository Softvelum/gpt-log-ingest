# Proxy tunnel request was not fully sent

Product: Nimble Streamer
Component: Panel synchronization and proxy transport
Logging function: log_e
Log level: error
Documentation ID: log_e-74fb4b4ca69af915

## Log pattern

```text
failed to send CONNECT request to proxy '%s:%d sent=%d errno=%d
```

## Parameters

- Argument 1 — Host: Destination host or proxy host, text.
- Argument 2 — Port: TCP destination port number, integer.
- Argument 3 — Send result: Bytes sent, or negative socket error return, integer.
- Argument 4 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send CONNECT request to proxy 'panel.example:443 sent=-1 errno=5
```

## Short description

Proxy tunnel request was not fully sent.

## Trigger condition

Sending the CONNECT request returns a byte count different from the full request length.

## Explanation

Tunnel setup fails without retrying a partial request.

## Possible causes

- The proxy exchange failed at the stated send, read, or parse step.

## What to check

1. Check the proxy address and response, and the socket error where reported.

## Recommended action

Restore a valid reachable HTTP CONNECT proxy.
