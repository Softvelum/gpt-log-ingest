# Stream-check event wait failed

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-c0f49dbef98b740b

## Log pattern

```text
epoll_wait epfd=%d failed (errno=%d)
```

## Parameters

- Argument 1 — selector: Event queue descriptor.
- Argument 2 — error: Operating system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
epoll_wait epfd=10 failed (errno=9)
```

## Short description

Stream-check event wait failed.

## Trigger condition

The Linux event wait returns a failure other than interruption by a signal.

## Explanation

The loop logs the error, checks task deadlines, and continues while tasks remain.

## Possible causes

- The operating system rejected the event wait.

## What to check

1. Check the error and selector lifetime.

## Recommended action

Investigate repeated wait failures that prevent task processing.
