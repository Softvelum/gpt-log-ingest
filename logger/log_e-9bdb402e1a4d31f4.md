# Closure event from an unowned socket

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-9bdb402e1a4d31f4

## Log pattern

```text
kEventConnClosed for unknown s=%d
```

## Parameters

- Argument 1 — Socket descriptor: Integer identifying the affected local socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
kEventConnClosed for unknown s=17
```

## Short description

Closure event from an unowned socket.

## Trigger condition

A connection-closed event matches neither the client nor origin socket in a handled session phase.

## Explanation

The unexpected descriptor is logged without immediate cleanup in that branch.

## Possible causes

- A connection-closed event matches neither the client nor origin socket in a handled session phase.

## What to check

1. Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.

## Recommended action

Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.
