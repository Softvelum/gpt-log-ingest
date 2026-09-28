# Child request buffer is full

Product: Nimble Streamer
Component: Child process communication
Logging function: log_e
Log level: error
Documentation ID: log_e-d73215cddfcbb76c

## Log pattern

```text
output buffer exhausted. Too many messages
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
output buffer exhausted. Too many messages
```

## Short description

Child request buffer is full.

## Trigger condition

A session is still connecting and cannot queue another pending request.

## Explanation

The current connection attempt fails; its queued messages are discarded during reset.

## Possible causes

- Queued requests exceed the configured output buffer capacity before the child becomes connected.

## What to check

1. Check child connection time and the volume and size of pending messages.

## Recommended action

Resolve the reported child communication problem before retrying.
