# Remote log batch prepared

Product: Nimble Streamer
Component: Remote log reporting
Logging function: log_v
Log level: verbose
Documentation ID: log_v-caa777a8d78c37b0

## Log pattern

```text
prepared log request:
%s
```

## Parameters

- Argument 1 — Request body: Serialized request body for remote log reporting; security-sensitive details are omitted.

## Log examples

A complete request example is omitted because it contains panel authentication material.

## Short description

Remote log batch prepared.

## Trigger condition

Queued records have been serialized, reporting is enabled, and a socket to the panel has opened.

## Explanation

A queued remote log request has been prepared for transmission; this message does not confirm delivery.

## Related log messages

- [Remote log synchronization failed](log_e-af35d0514f1d0617.md)
- [Remote log response received](log_v-12298e1205567192.md)
