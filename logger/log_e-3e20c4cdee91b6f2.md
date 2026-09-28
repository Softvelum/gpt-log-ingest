# Proxy tunnel response could not be parsed

Product: Nimble Streamer
Component: Panel synchronization and proxy transport
Logging function: log_e
Log level: error
Documentation ID: log_e-3e20c4cdee91b6f2

## Log pattern

```text
failed to parse response from proxy '%s:%d'
```

## Parameters

- Argument 1 — Host: Destination host or proxy host, text.
- Argument 2 — Port: TCP destination port number, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse response from proxy 'panel.example:443'
```

## Short description

Proxy tunnel response could not be parsed.

## Trigger condition

The HTTP response parser rejects the accumulated proxy response.

## Explanation

Tunnel setup fails.

## Possible causes

- The proxy exchange failed at the stated send, read, or parse step.

## What to check

1. Check the proxy address and response, and the socket error where reported.

## Recommended action

Restore a valid reachable HTTP CONNECT proxy.
