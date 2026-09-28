# Origin read requested in an invalid phase

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-f37bf0c94fc6ee9f

## Log pattern

```text
OnOriginResp invalid session state=%d
```

## Parameters

- Argument 1 — Session state: Integer identifying the HTTP client session phase: 0 reading a request, 1 sending a response, 2 connecting to origin, 3 sending to origin, 4 reading origin headers, 5 reading an origin playlist, 6 reading origin media, 29 shutting down, or 30 finished; intermediate values identify pending asynchronous work.

## Log examples

Illustrative examples; these are not captured server logs.

```text
OnOriginResp invalid session state=4
```

## Short description

Origin read requested in an invalid phase.

## Trigger condition

The origin-read handler is entered outside body, header, or playlist reception.

## Explanation

A shutdown response is prepared instead of reading the origin socket.

## Possible causes

- The origin-read handler is entered outside body, header, or playlist reception.

## What to check

1. Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.

## Recommended action

Correlate the socket and surrounding connection events; repeated unexpected events require investigation of session or socket ownership.
