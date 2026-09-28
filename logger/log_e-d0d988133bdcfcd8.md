# WHIP publish-signature policy missing

Product: Nimble Streamer
Component: WHIP publishing authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-d0d988133bdcfcd8

## Log pattern

```text
WHIP publish not authorized as application has no publish settings stream=[%s/%s] ip=[%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.
- Argument 3 — Client address: Publisher IP address supplied with the request, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WHIP publish not authorized as application has no publish settings stream=[live/camera] ip=[192.0.2.10]
```

## Short description

WHIP publish-signature policy missing.

## Trigger condition

The WHIP publish-signature authorization branch reports that publish authorization is not required for the requested application.

## Explanation

The WHIP session is not created; the caller receives no session.

## Possible causes

- The request reached signature authorization for an application without applicable publish settings.

## What to check

1. Check that the application name and publish-signature policy match the publisher request.

## Recommended action

Configure the intended publish authorization policy or correct the requested application.
