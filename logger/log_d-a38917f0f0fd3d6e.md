# Socket event monitor closed

Product: Nimble Streamer
Component: Socket event monitoring
Logging function: log_d
Log level: debug
Documentation ID: log_d-a38917f0f0fd3d6e

## Log pattern

```text
closed epoll epfd=%d
```

## Parameters

- Argument 1 — Event monitor: Linux event-monitor descriptor, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
closed epoll epfd=10
```

## Short description

Socket event monitor closed.

## Trigger condition

A positive Linux event-monitor descriptor closes successfully.

## Explanation

The stored descriptor is invalidated; monitoring through it ends.

## Related log messages

- [Socket event monitor close failed](log_e-f379e7759ef5f57e.md)
