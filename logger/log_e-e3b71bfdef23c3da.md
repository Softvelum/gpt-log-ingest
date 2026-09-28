# Socket event monitor creation failed

Product: Nimble Streamer
Component: Socket event monitoring
Logging function: log_e
Log level: error
Documentation ID: log_e-e3b71bfdef23c3da

## Log pattern

```text
epoll_create failed errno=%d
```

## Parameters

- Argument 1 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
epoll_create failed errno=5
```

## Short description

Socket event monitor creation failed.

## Trigger condition

Linux cannot create the event-monitor descriptor.

## Explanation

Construction throws an exception, so this monitor is unavailable.

## Possible causes

- The operating system refused creation of an event-monitor descriptor.

## What to check

1. Check the operating system error and available descriptor resources.

## Recommended action

Resolve the reported operating system error.
