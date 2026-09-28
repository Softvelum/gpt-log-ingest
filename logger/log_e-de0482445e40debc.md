# Cannot close SRT socket

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-de0482445e40debc

## Log pattern

```text
failed to close socket s=%d: %s
```

## Parameters

- Argument 1 — Socket: SRT socket or group identifier.
- Argument 2 — Error: SRT library error description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to close socket s=100: Invalid socket ID
```

## Short description

Cannot close SRT socket.

## Trigger condition

The SRT library returns failure while closing an untagged socket.

## Explanation

The close helper returns failure; this message does not prove the socket was closed.

## Possible causes

- The SRT library returns failure while closing an untagged socket.

## What to check

1. Inspect the error and earlier socket lifecycle messages.
