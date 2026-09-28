# External request received an unsupported socket event

Product: Nimble Streamer
Component: External HTTP requests
Logging function: log_e
Log level: error
Documentation ID: log_e-89bd7896d4e0b1db

## Log pattern

```text
requestor session p=%p unknown event for s=%d ev=0x%x
```

## Parameters

- Argument 1 — address: Request session memory address.
- Argument 2 — socket: Peer socket descriptor.
- Argument 3 — events: Socket event flags in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
requestor session p=0x1000 unknown event for s=12 ev=0x0
```

## Short description

External request received an unsupported socket event.

## Trigger condition

The event contains no input, output, recognized closure, or recognized error condition.

## Explanation

The request session is marked finished; destruction reports failure if no result callback has already completed.

## Possible causes

- The event flags do not match supported request processing events.

## What to check

1. Compare the event mask and previous request state.

## Recommended action

Investigate unexpected event delivery.
