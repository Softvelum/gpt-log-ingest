# Publishing signature lacks a publisher identity

Product: Nimble Streamer
Component: Publisher authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-fe12a5a6f57f55ae

## Log pattern

```text
invalid(id) publishsign=[%s], for [%s][%s]
```

## Parameters

- Argument 1 — signature: Original encoded publishing authorization value.
- Argument 2 — stream: Application and stream path being published, beginning with a slash.
- Argument 3 — client: Publishing client IP address.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid(id) publishsign=[c2lnbj14JmlwPTE5Mi4wLjIuMTA=], for [/live/camera][192.0.2.10]
```

## Short description

Publishing signature lacks a publisher identity.

## Trigger condition

The decoded fields have no usable publisher identity.

## Explanation

Signature validation returns failure before publisher registration or a remote authorization request.

## Possible causes

- The decoded fields have no usable publisher identity.

## What to check

1. Compare the client signature fields with the intended publisher, stream, password, and optional client address.

## Recommended action

Generate a valid publishing signature for the actual request.
