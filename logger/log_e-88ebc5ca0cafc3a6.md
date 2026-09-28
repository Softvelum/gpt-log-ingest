# Socket could not be removed from event monitoring

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_e
Log level: error
Documentation ID: log_e-88ebc5ca0cafc3a6

## Log pattern

```text
BaseSession(%p) failed to delete %s s=%d from selector fd=%d
```

## Parameters

- Argument 1 — Session: Session address used for diagnostic correlation.
- Argument 2 — Peer role: Text identifying the socket role.
- Argument 3 — Socket: Integer socket descriptor.
- Argument 4 — Event selector: Integer descriptor identifying the event selector.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseSession(0x1000) failed to delete client s=12 from selector fd=7
```

## Short description

Socket could not be removed from event monitoring.

## Trigger condition

Removing the socket from its event selector returns failure.

## Explanation

The failed removal is reported; this helper does not throw or retry the removal.

## Possible causes

- The event selector rejected socket removal.

## What to check

1. Compare the socket and selector descriptors with related connection events.
