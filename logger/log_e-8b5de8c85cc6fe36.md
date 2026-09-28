# Unexpected HTTP writable event

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-8b5de8c85cc6fe36

## Log pattern

```text
ClientSession(%p cs=%d os=%d) unexpected kEventOut for s=%d (state=%d)
```

## Parameters

- Argument 1 — Session address: Pointer identifying this client session within the process.
- Argument 2 — Client socket: Integer identifying the local client socket.
- Argument 3 — Origin socket: Integer identifying the local origin socket.
- Argument 4 — Socket descriptor: Integer identifying the affected local socket.
- Argument 5 — Session state: Integer identifying the HTTP client session phase: 0 reading a request, 1 sending a response, 2 connecting to origin, 3 sending to origin, 4 reading origin headers, 5 reading an origin playlist, 6 reading origin media, 29 shutting down, or 30 finished; intermediate values identify pending asynchronous work.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ClientSession(0x1234 cs=18 os=17) unexpected kEventOut for s=17 (state=4)
```

## Short description

Unexpected HTTP writable event.

## Trigger condition

A writable event reaches a state that has no output handler, including shutdown on a socket other than the TLS client.

## Explanation

The event and both owned descriptors are recorded; the shown branches normally make no state change.

## Possible causes

- A writable event reaches a state that has no output handler, including shutdown on a socket other than the TLS client.

## What to check

1. Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.

## Recommended action

Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.
