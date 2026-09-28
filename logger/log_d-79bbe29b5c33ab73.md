# Cache notification subscription removal processed

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-79bbe29b5c33ab73

## Log pattern

```text
[WrkT::RemO] key='%s' p=%p
```

## Parameters

- Argument 1 — Cache key: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::RemO] key='example-item' p=0x1000
```

## Short description

Cache notification subscription removal processed.

## Trigger condition

Removal from an existing cache waiting-session set leaves other sessions waiting.

## Explanation

The remaining sessions continue to receive cache updates. The message does not establish that the supplied session was present before removal.
