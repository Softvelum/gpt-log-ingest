# IP range prefix length conversion failed

Product: Nimble Streamer
Component: IP access ranges
Logging function: log_e
Log level: error
Documentation ID: log_e-419b8a27bfe8e9a9

## Log pattern

```text
failed to get bit_num from range='%s'
```

## Parameters

- Argument 1 — Range: Text containing the individual address and prefix-length expression.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get bit_num from range='192.0.2.0/x'
```

## Short description

IP range prefix length conversion failed.

## Trigger condition

For an IPv4 or IPv6 range, prefix-length conversion yields zero while the system error value indicates an invalid argument.

## Explanation

The allocated range keeps its zero endpoints and processing continues with later ranges. The displayed condition is narrower than a complete validity check of the prefix length.

## Possible causes

- The checked prefix conversion and system-error condition indicated an invalid argument.

## What to check

1. Check the numeric prefix-length portion of the displayed range.

## Recommended action

Supply the intended numeric prefix length.

## Related log messages

- [IP range lacks address and prefix parts](log_e-f7e3fe80b5a9f6bb.md)
