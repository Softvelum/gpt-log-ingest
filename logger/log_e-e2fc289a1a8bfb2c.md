# Unknown socket sent input during shutdown

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-e2fc289a1a8bfb2c

## Log pattern

```text
ClientSession(%p cs=%d os=%d) unexpected data from unknown s=%d (state=%d)
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
ClientSession(0x1234 cs=18 os=17) unexpected data from unknown s=17 (state=4)
```

## Short description

Unknown socket sent input during shutdown.

## Trigger condition

An input event during shutdown matches neither the client nor origin descriptor.

## Explanation

The descriptor mismatch is logged and the session is marked finished.

## Possible causes

- An input event during shutdown matches neither the client nor origin descriptor.

## What to check

1. Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.

## Recommended action

Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.
