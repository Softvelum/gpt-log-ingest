# Cache change delivered to a waiting session

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-e978db6516ed06ca

## Log pattern

```text
[WrkT::PN] key='%s' p=%p
```

## Parameters

- Argument 1 — Cache key: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::PN] key='example-item' p=0x1000
```

## Short description

Cache change delivered to a waiting session.

## Trigger condition

A queued cache change has matching waiting sessions.

## Explanation

The identified session is notified next, then deleted if notification handling finishes it.
