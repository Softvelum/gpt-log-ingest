# Windows service dispatcher could not start

Product: Nimble Streamer
Component: Windows service
Logging function: log_e
Log level: error
Documentation ID: log_e-0a5a0cb907abaa5e

## Log pattern

```text
can't start service ctrl dispatcher %d
```

## Parameters

- Argument 1 — System error: Numeric Windows error returned immediately after the dispatcher failure.

## Log examples

Illustrative examples; these are not captured server logs.

```text
can't start service ctrl dispatcher 5
```

## Short description

Windows service dispatcher could not start.

## Trigger condition

Windows service mode is selected and starting the service control dispatcher fails.

## Explanation

Startup exits with a failure status instead of entering the service runtime. Only Windows builds contain this path.

## Possible causes

- The operating system rejected service dispatcher startup.

## What to check

1. Use the reported system error to determine why service startup was rejected.
