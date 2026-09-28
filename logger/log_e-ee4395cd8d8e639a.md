# Publishing signature digest does not match

Product: Nimble Streamer
Component: Publisher authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-ee4395cd8d8e639a

## Log pattern

```text
invalid(signature match) publishsign=[%s], for [%s][%s]
```

## Parameters

- Argument 1 — signature: Original encoded publishing authorization value.
- Argument 2 — stream: Application and stream path being published, beginning with a slash.
- Argument 3 — client: Publishing client IP address.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid(signature match) publishsign=[aWQ9cHVibGlzaGVyJnNpZ249eA==], for [/live/camera][192.0.2.10]
```

## Short description

Publishing signature digest does not match.

## Trigger condition

The supplied digest differs from the digest computed from publisher identity, stream path, password, and optional IP.

## Explanation

Signature validation returns failure before publisher registration or a remote authorization request.

## Possible causes

- The supplied digest differs from the digest computed from publisher identity, stream path, password, and optional IP.

## What to check

1. Compare the client signature fields with the intended publisher, stream, password, and optional client address.

## Recommended action

Generate a valid publishing signature for the actual request.
