# RIST output local connection receives a terminal event

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-cb1d7dedaa95a836

## Log pattern

```text
rist sender connection closed s=%d event=0x%x, id=%s
```

## Parameters

- Argument 1 — Socket: Previously attached local helper socket descriptor.
- Argument 2 — Events: Complete socket event bitmask in hexadecimal.
- Argument 3 — Identifier: Configured RIST sender identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rist sender connection closed s=42 event=0x2000, id=output1
```

## Short description

RIST output local connection receives a terminal event.

## Trigger condition

The attached local helper socket has an error, closure, or hangup event after any input/output work completes.

## Explanation

The output enters shutdown. This describes the local helper connection, not proof of a remote RIST peer connection.

## Possible causes

- The attached local helper socket has an error, closure, or hangup event after any input/output work completes.

## What to check

1. Correlate the local socket and input/output identifier with helper startup and connection errors.

## Recommended action

No action is needed for an expected shutdown; investigate unexpected repeated helper closures.
