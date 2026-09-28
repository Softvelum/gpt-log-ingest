# Windows service execution has ended

Product: Nimble Streamer
Component: Windows service
Logging function: log_i
Log level: info
Documentation ID: log_i-c23a0bd524deb194

## Log pattern

```text
service stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
service stopped
```

## Short description

Windows service execution has ended.

## Trigger condition

The Windows service returns from server initialization, execution and cleanup.

## Explanation

The service stops accepting stop and shutdown controls, then submits its stopped state to the operating system.

## Related log messages

- [Windows service status update failed](log_e-3944d92173d2044a.md)
- [Windows service is entering its running state](log_i-e76e988b31b112eb.md)
