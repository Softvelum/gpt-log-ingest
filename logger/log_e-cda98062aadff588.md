# Stream checker cannot create its event queue

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-cda98062aadff588

## Log pattern

```text
streamCheckerThread stopped .cannot create epoll [%d]
```

## Parameters

- Argument 1 — error: Operating system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
streamCheckerThread stopped .cannot create epoll [24]
```

## Short description

Stream checker cannot create its event queue.

## Trigger condition

Linux event queue creation fails while initializing either checker.

## Explanation

Initialization returns failure and the requesting checker thread exits before processing tasks.

## Possible causes

- The operating system could not allocate the event queue.

## What to check

1. Check the error and file-descriptor availability.

## Recommended action

Resolve the operating system resource problem.
