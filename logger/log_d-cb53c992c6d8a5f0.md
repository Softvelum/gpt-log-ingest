# Socket closed

Product: Nimble Streamer
Component: Socket communication
Logging function: log_d
Log level: debug
Documentation ID: log_d-cb53c992c6d8a5f0

## Log pattern

```text
%s socket s=%d closed
```

## Parameters

- Argument 1 — Socket label: Caller-supplied descriptive socket label, text.
- Argument 2 — Socket: Socket descriptor, integer; no unit.

The descriptor is an operating system socket or an SRT handle according to the caller.

## Log examples

Illustrative examples; these are not captured server logs.

```text
input socket s=12 closed
```

## Short description

Socket closed.

## Trigger condition

Closing a normal operating system socket or an SRT socket succeeds.

## Explanation

The respective close helper returns success.
