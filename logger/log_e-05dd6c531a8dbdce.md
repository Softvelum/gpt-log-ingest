# RTMP publishing is not authorized

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-05dd6c531a8dbdce

## Log pattern

```text
publish not authorized for '%s/%s'
```

## Parameters

- Argument 1 — Application: Application receiving the publish request.
- Argument 2 — Stream: Stream name after query parameters are removed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
publish not authorized for 'live/camera'
```

## Short description

RTMP publishing is not authorized.

## Trigger condition

Publishing reaches an authorization branch that requires application credentials that have not been accepted, or the publish authorization result explicitly denies access.

## Explanation

The stream is not registered and the publish command fails, stopping connection processing. The log does not identify which authorization branch denied access.

## Possible causes

- Publishing reaches an authorization branch that requires application credentials that have not been accepted, or the publish authorization result explicitly denies access.

## What to check

1. Check the application publishing credentials, publish signature, and authorization decision for this stream.

## Recommended action

Supply accepted publishing credentials or correct the authorization policy for the intended publisher.
