# RTMP publish-signature authorization fails

Product: Nimble Streamer
Component: RTMP connection transport
Logging function: log_e
Log level: error
Documentation ID: log_e-7b8a71c7bb9802fe

## Log pattern

```text
publish auth failed for ip='%s' url='%s/%s'
```

## Parameters

- Argument 1 — Publisher IP: Original publisher address from an accepted PROXY header when available, otherwise the connected peer address.
- Argument 2 — Application: Target publishing application.
- Argument 3 — Stream: Target publishing stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
publish auth failed for ip='192.0.2.25' url='live/camera'
```

## Short description

RTMP publish-signature authorization fails.

## Trigger condition

Publish-signature processing returns an immediate failure for the incoming publisher.

## Explanation

The connection is shut down and publishing remains unauthorized. This message identifies the publisher IP and target, but not the detailed authorization reason.

## Possible causes

- Publish-signature processing returns an immediate failure for the incoming publisher.

## What to check

1. Check the signature, publishing authorization rules, and earlier authorization diagnostics for this target.

## Recommended action

Supply an accepted signature or correct the intended publishing authorization configuration.
