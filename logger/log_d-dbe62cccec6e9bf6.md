# Last session stopped waiting for a cache item

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-dbe62cccec6e9bf6

## Log pattern

```text
[WrkT::RemO] key='%s' p=%p empty set
```

## Parameters

- Argument 1 — Cache key: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::RemO] key='example-item' p=0x1000 empty set
```

## Short description

Last session stopped waiting for a cache item.

## Trigger condition

A cache waiting-session set becomes empty after removal is attempted.

## Explanation

The worker unregisters from the cache and removes the empty set.
