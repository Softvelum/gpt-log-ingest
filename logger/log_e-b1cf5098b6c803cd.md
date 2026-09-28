# Publishing signature has the wrong field count

Product: Nimble Streamer
Component: Publisher authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-b1cf5098b6c803cd

## Log pattern

```text
invalid publishsign=[%s] for [%s][%s]
```

## Parameters

- Argument 1 — signature: Original encoded publishing authorization value.
- Argument 2 — stream: Application and stream path being published, beginning with a slash.
- Argument 3 — client: Publishing client IP address.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid publishsign=[aWQ9cHVibGlzaGVy] for [/live/camera][192.0.2.10]
```

## Short description

Publishing signature has the wrong field count.

## Trigger condition

Decoded publishing signature data does not contain exactly two or three ampersand-separated fields.

## Explanation

Signature validation returns failure before publisher registration or a remote authorization request.

## Possible causes

- Decoded publishing signature data does not contain exactly two or three ampersand-separated fields.

## What to check

1. Compare the client signature fields with the intended publisher, stream, password, and optional client address.

## Recommended action

Generate a valid publishing signature for the actual request.
