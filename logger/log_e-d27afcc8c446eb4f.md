# WHIP application credentials unavailable

Product: Nimble Streamer
Component: WHIP publishing authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-d27afcc8c446eb4f

## Log pattern

```text
WHIP publish not authorized as application has no login/password set stream=[%s/%s] ip=[%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.
- Argument 3 — Client address: Publisher IP address supplied with the request, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WHIP publish not authorized as application has no login/password set stream=[live/camera] ip=[192.0.2.10]
```

## Short description

WHIP application credentials unavailable.

## Trigger condition

A WHIP request uses application credentials without a publish signature, signature-free publishing is disabled, and the application reports that login/password authorization is not required.

## Explanation

The WHIP session is not created; the caller receives no session.

## Possible causes

- The application has no matching login/password authorization setup for this request.

## What to check

1. Check the application named in the message and its publishing login/password settings.

## Recommended action

Configure the application credentials expected by the publisher or use the intended publish-signature policy.
