# Process task monitor cannot create its event queue

Product: Nimble Streamer
Component: External process supervision
Logging function: log_e
Log level: error
Documentation ID: log_e-d3c4dca263637ce8

## Log pattern

```text
TaskManagerThread epoll_create1 failed errno=%d
```

## Parameters

- Argument 1 — error: Operating system error number from event queue creation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
TaskManagerThread epoll_create1 failed errno=24
```

## Short description

Process task monitor cannot create its event queue.

## Trigger condition

Linux event queue creation fails at task-monitor startup.

## Explanation

The task-monitor thread returns before entering the loop that starts, monitors, and restarts external tasks.

## Possible causes

- The operating system could not allocate the event queue.

## What to check

1. Check the error number and available file descriptors.

## Recommended action

Resolve the operating system resource failure before restarting task monitoring.
