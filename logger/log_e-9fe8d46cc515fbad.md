# Additional stream-check connection was rejected

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-9fe8d46cc515fbad

## Log pattern

```text
failed to connect s=%d '%s:%d' [%d] task='%s
```

## Parameters

- Argument 1 — socket: Socket descriptor for the stream-check connection.
- Argument 2 — host: Origin host name from the task URL.
- Argument 3 — port: Origin TCP port.
- Argument 4 — error: Operating system error number.
- Argument 5 — task: Panel-supplied stream-check task identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect s=12 'media.example:443' [111] task='check-a
```

## Short description

Additional stream-check connection was rejected.

## Trigger condition

Connecting to another resolved address does not return the expected in-progress result.

## Explanation

The new socket is closed and the alternate attempt fails. Immediate successful connection also enters this branch, so its error value alone is not conclusive.

## Possible causes

- The connect result differs from the expected asynchronous connection state.

## What to check

1. Check origin reachability and the task connection sequence.

## Recommended action

Investigate the connection result and origin availability.
