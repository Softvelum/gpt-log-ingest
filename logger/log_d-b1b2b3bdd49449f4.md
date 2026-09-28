# Windows datagram receive failed

Product: Nimble Streamer
Component: Network input receive
Logging function: log_d
Log level: debug
Documentation ID: log_d-b1b2b3bdd49449f4

## Log pattern

```text
failed to recv from s=%d (ws_err=%d)
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Windows socket error: Windows socket error code, integer; no unit.

The QUIC occurrence reads a datagram and ancillary addressing data; the MPEG-TS occurrence reads transport-stream bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to recv from s=12 (ws_err=10054)
```

## Short description

Windows datagram receive failed.

## Trigger condition

A QUIC datagram receive or MPEG-TS UDP input receive returns an error other than would-block on Windows.

## Explanation

QUIC stops processing the current input event while retaining the listener; MPEG-TS UDP input reports failure to its caller.

## Possible causes

- Windows reports a hard receive error on the socket.

## What to check

1. Interpret the Windows socket error code and check the affected listener or UDP source.

## Recommended action

Correct the reported socket problem and restore the affected input if it stops.
