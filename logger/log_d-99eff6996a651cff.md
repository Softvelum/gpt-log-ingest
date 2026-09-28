# QUIC event has no listener address

Product: Nimble Streamer
Component: QUIC datagram listener
Logging function: log_d
Log level: debug
Documentation ID: log_d-99eff6996a651cff

## Log pattern

```text
event for unknown fd=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
event for unknown fd=12
```

## Short description

QUIC event has no listener address.

## Trigger condition

An input-ready socket descriptor is absent from the QUIC listener address map.

## Explanation

The event is ignored without reading a packet or dispatching it to a QUIC worker.

## Possible causes

- Socket event delivery and the registered listener-address map are inconsistent.

## What to check

1. Check listener creation/removal events for the descriptor.

## Recommended action

Investigate repeated events for a descriptor that is no longer registered.
