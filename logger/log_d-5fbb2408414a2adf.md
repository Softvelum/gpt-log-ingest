# Session begins waiting for cache changes

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-5fbb2408414a2adf

## Log pattern

```text
[WrkT::AddO] key='%s' p=%p
```

## Parameters

- Argument 1 — Cache key: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::AddO] key='example-item' p=0x1000
```

## Short description

Session begins waiting for cache changes.

## Trigger condition

A session asks to observe a cache item.

## Explanation

An existing waiting-session set is updated, or the worker first registers with the cache. The message is emitted before success or duplicate detection.
