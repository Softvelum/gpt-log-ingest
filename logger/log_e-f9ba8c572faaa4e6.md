# Child process exited before connecting

Product: Nimble Streamer
Component: Child process communication
Logging function: log_e
Log level: error
Documentation ID: log_e-f9ba8c572faaa4e6

## Log pattern

```text
child process stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
child process stopped
```

## Short description

Child process exited before connecting.

## Trigger condition

The connecting session exceeds its five-second deadline and the child is absent or no longer running.

## Explanation

The attempt fails and its session is stopped. Persistent workers reset; replaceable workers exit.

## Possible causes

- The launched child has stopped before its session became connected.

## What to check

1. Check the child startup diagnostics preceding this timeout.

## Recommended action

Resolve the child startup failure before retrying.

## Related log messages

- [Child process connection timed out](log_e-f7c504048c309654.md)
