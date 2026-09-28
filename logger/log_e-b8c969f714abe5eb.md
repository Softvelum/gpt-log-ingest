# Unexpected successful authorization completion

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-b8c969f714abe5eb

## Log pattern

```text
invalid session state=%d in ClientSession::OnAuthSuccess
```

## Parameters

- Argument 1 — Session state: Integer identifying the HTTP client session phase: 0 reading a request, 1 sending a response, 2 connecting to origin, 3 sending to origin, 4 reading origin headers, 5 reading an origin playlist, 6 reading origin media, 29 shutting down, or 30 finished; intermediate values identify pending asynchronous work.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid session state=4 in ClientSession::OnAuthSuccess
```

## Short description

Unexpected successful authorization completion.

## Trigger condition

An external authorization request completes as successful while the HTTP client is no longer waiting for authorization.

## Explanation

The pending authorization identifier is cleared and the client session is marked finished.

## Possible causes

- An external authorization request completes as successful while the HTTP client is no longer waiting for authorization.

## What to check

1. Correlate authorization timing with client closure or other session transitions; the result arrived in an incompatible phase.

## Recommended action

Correlate authorization timing with client closure or other session transitions; the result arrived in an incompatible phase.
