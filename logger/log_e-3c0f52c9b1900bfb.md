# RTMP response contains an unreadable extra field

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-3c0f52c9b1900bfb

## Log pattern

```text
cannot process AMF. cannot skip %s
```

## Parameters

- Argument 1 — Field name: Name of the AMF response field whose value could not be skipped.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot process AMF. cannot skip description
```

## Short description

RTMP response contains an unreadable extra field.

## Trigger condition

An unhandled field cannot be skipped in a connect result, Limelight authentication response, play status, or publish status.

## Explanation

Connect, play, and publish response handlers fail, stopping connection processing. Limelight authentication parsing returns without completing any later challenge fields; its enclosing error-response path also ends the connection.

## Possible causes

- An unhandled field cannot be skipped in a connect result, Limelight authentication response, play status, or publish status.

## What to check

1. Inspect the named field and its AMF value in the remote response.

## Recommended action

Correct unsupported or malformed field encoding at the remote endpoint.
