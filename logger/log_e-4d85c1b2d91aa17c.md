# Adobe authentication challenge names a different user

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-4d85c1b2d91aa17c

## Log pattern

```text
server returned wrong user(expected=%s,received=%s) for '%s'
```

## Parameters

- Argument 1 — Expected user: Configured publishing login.
- Argument 2 — Returned user: User query value in the remote authentication challenge.
- Argument 3 — Stream URL: RTMP or RTMPS URL identifying the outbound stream connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
server returned wrong user(expected=publisher,received=otheruser) for 'rtmp://destination.example:1935/live/camera'
```

## Short description

Adobe authentication challenge names a different user.

## Trigger condition

A destination Adobe authentication challenge requests authentication but its user value differs from the configured publishing login.

## Explanation

The challenge is discarded before the final authentication state is entered. The enclosing error-response path ends the current connection.

## Possible causes

- A destination Adobe authentication challenge requests authentication but its user value differs from the configured publishing login.

## What to check

1. Compare the expected and returned users with the destination account configuration.

## Recommended action

Use matching publishing credentials and destination authentication settings.
