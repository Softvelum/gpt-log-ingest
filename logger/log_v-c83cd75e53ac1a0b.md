# Input arrived during a pending HTTP operation

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c83cd75e53ac1a0b

## Log pattern

```text
Unexpected data from s=%d (state=%d)
```

## Parameters

- Argument 1 — Socket descriptor: Integer identifying the affected local socket.
- Argument 2 — Session state: Integer identifying the HTTP client session phase: 0 reading a request, 1 sending a response, 2 connecting to origin, 3 sending to origin, 4 reading origin headers, 5 reading an origin playlist, 6 reading origin media, 29 shutting down, or 30 finished; intermediate values identify pending asynchronous work.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Unexpected data from s=17 (state=4)
```

## Short description

Input arrived during a pending HTTP operation.

## Trigger condition

An input event reaches a session phase with no input handler.

## Explanation

The default shutdown response is prepared.

## Possible causes

- An input event reaches a session phase with no input handler.

## What to check

1. Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.

## Recommended action

Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.
