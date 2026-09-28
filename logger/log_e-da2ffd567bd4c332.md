# Windows service control handler registration failed

Product: Nimble Streamer
Component: Windows service
Logging function: log_e
Log level: error
Documentation ID: log_e-da2ffd567bd4c332

## Log pattern

```text
can't register service ctrl handler error=%d
```

## Parameters

- Argument 1 — System error: Numeric Windows error from handler registration.

## Log examples

Illustrative examples; these are not captured server logs.

```text
can't register service ctrl handler error=5
```

## Short description

Windows service control handler registration failed.

## Trigger condition

Windows service initialization receives no valid handle after registering its control handler.

## Explanation

The service entry routine returns immediately without reporting a running state or starting the server.

## Possible causes

- The operating system rejected handler registration.

## What to check

1. Inspect the reported Windows error.

## Related log messages

- [Windows service is entering its running state](log_i-e76e988b31b112eb.md)
