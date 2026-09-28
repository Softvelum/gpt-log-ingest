# Management connection close or error event

Product: Nimble Streamer
Component: HTTP management API
Logging function: log_d
Log level: debug
Documentation ID: log_d-8607d0bedcd8a2e8

## Log pattern

```text
ConnHup|ConnClosed|Error received for s=%d ev=0x%x
```

## Parameters

- Argument 1 — socket: Client socket descriptor.
- Argument 2 — event\_mask: Received event flags in hexadecimal.

## Log examples

No numerical example is supplied because the relevant event-flag values were not established from the inspected source.

## Short description

Management connection close or error event.

## Trigger condition

The event mask includes connection hangup, connection closed, or error.

## Explanation

The client enters its completed shutdown state and returns without processing input or output. The owning management thread then removes and deletes that client.
