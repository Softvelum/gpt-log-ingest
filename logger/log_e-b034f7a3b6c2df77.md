# Origin connection event has the wrong socket

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-b034f7a3b6c2df77

## Log pattern

```text
%s s=%d <> sock=%d
```

## Parameters

- Argument 1 — Socket role: Text label of the expected socket.
- Argument 2 — Origin socket: Integer identifying the local origin socket.
- Argument 3 — Event socket: Integer descriptor supplied with the writable event.

## Log examples

Illustrative examples; these are not captured server logs.

```text
origin s=17 <> sock=19
```

## Short description

Origin connection event has the wrong socket.

## Trigger condition

During origin connection establishment, the writable-event descriptor differs from the stored origin descriptor.

## Explanation

The origin is closed and an HTTP 500 response is prepared for the client.

## Possible causes

- During origin connection establishment, the writable-event descriptor differs from the stored origin descriptor.

## What to check

1. Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.

## Recommended action

Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.
