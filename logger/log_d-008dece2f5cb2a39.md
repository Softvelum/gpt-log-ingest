# HTTP connection session created

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-008dece2f5cb2a39

## Log pattern

```text
created client session=%p (s=%d)
```

## Parameters

- Argument 1 — Session: Connection-session address used for diagnostic correlation.
- Argument 2 — Socket: Integer socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
created client session=0x1000 (s=12)
```

## Short description

HTTP connection session created.

## Trigger condition

A session has been constructed for a newly accepted HTTP connection or for an HTTP/2 upgrade.

## Explanation

For a new connection, active registration and event-monitor setup follow. For an HTTP/2 upgrade, the old session is marked finished and the new connection replaces its descriptor mapping.
