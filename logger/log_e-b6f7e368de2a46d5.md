# Closure event has no handler for this phase

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-b6f7e368de2a46d5

## Log pattern

```text
state=%d need to process kEventConnClosed for s=%d
```

## Parameters

- Argument 1 — Session state: Integer identifying the HTTP client session phase: 0 reading a request, 1 sending a response, 2 connecting to origin, 3 sending to origin, 4 reading origin headers, 5 reading an origin playlist, 6 reading origin media, 29 shutting down, or 30 finished; intermediate values identify pending asynchronous work.
- Argument 2 — Socket descriptor: Integer identifying the affected local socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
state=4 need to process kEventConnClosed for s=17
```

## Short description

Closure event has no handler for this phase.

## Trigger condition

A connection-closed event arrives in a session state not covered by the closure dispatcher.

## Explanation

The state and descriptor are logged; this branch makes no state change.

## Possible causes

- A connection-closed event arrives in a session state not covered by the closure dispatcher.

## What to check

1. Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.

## Recommended action

Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.
