# Authorization result has no waiting client

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_v
Log level: verbose
Documentation ID: log_v-8532bb36afc5b3db

## Log pattern

```text
auth client for request_id=%llu not found
```

## Parameters

- Argument 1 — Request identifier: Unsigned 64-bit authorization request identifier.

The identifier uses an unsigned-long-long conversion; exact underlying 64-bit type compatibility depends on the build.

## Log examples

Illustrative examples; these are not captured server logs.

```text
auth client for request_id=42 not found
```

## Short description

Authorization result has no waiting client.

## Trigger condition

A queued authorization result refers to a request identifier absent from the waiting-client map.

## Explanation

The result is ignored. This can occur when the notification was queued while its client was being deleted.

## Possible causes

- The waiting client has already been removed, including during client deletion.

## What to check

1. Compare the request identifier with the associated client deletion.
