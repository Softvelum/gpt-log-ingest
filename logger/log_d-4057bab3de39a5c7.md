# HTTP byte range has zero or omitted end

Product: Nimble Streamer
Component: HTTP media delivery
Logging function: log_d
Log level: debug
Documentation ID: log_d-4057bab3de39a5c7

## Log pattern

```text
parsed range %llu-%llu
```

## Parameters

- Argument 1 — Start: Parsed starting byte value, zero if omitted.
- Argument 2 — End: Parsed ending value, zero in this branch.

The unsigned 64-bit bounds use long-long conversions; exact typedef compatibility is platform dependent.

## Log examples

Illustrative examples; these are not captured server logs.

```text
parsed range 1024-0
```

## Short description

HTTP byte range has zero or omitted end.

## Trigger condition

A bytes range is parsed and its ending numeric value is zero.

## Explanation

The helper returns success. This message cannot distinguish an omitted end from an explicitly specified zero.

## Possible causes

- A bytes range is parsed and its ending numeric value is zero.

## What to check

1. Check the original Range header and subsequent range resolution.
