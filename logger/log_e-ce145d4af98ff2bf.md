# Initial stream-check connection was rejected

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-ce145d4af98ff2bf

## Log pattern

```text
failed to connect s=%d '%s:%d' [%d] task='%s'
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
failed to connect s=12 'media.example:443' [111] task='check-a'
```

## Short description

Initial stream-check connection was rejected.

## Trigger condition

The initial connect call does not return the expected in-progress result.

## Explanation

The task starts shutdown and returns connection failure. The branch also treats an immediate successful connect as failure, so the printed error is not conclusive in that unusual case.

## Possible causes

- The connect result differs from the expected asynchronous connection state.

## What to check

1. Check origin reachability and the task connection sequence.

## Recommended action

Investigate the connection result and origin availability.
