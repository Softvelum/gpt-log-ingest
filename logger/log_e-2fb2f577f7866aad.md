# System time moved backward during timer checking

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_e
Log level: error
Documentation ID: log_e-2fb2f577f7866aad

## Log pattern

```text
reverse time gap detected current=%lld last=%lld
```

## Parameters

- Argument 1 — Current time: Current wall-clock value in seconds since the system epoch.
- Argument 2 — Previous time: Wall-clock value from the preceding timer check.

The source passes platform time values to long-long conversions; exact type compatibility depends on the build.

## Log examples

Illustrative examples; these are not captured server logs.

```text
reverse time gap detected current=100 last=110
```

## Short description

System time moved backward during timer checking.

## Trigger condition

The current wall-clock time is earlier than the previous timer-check time.

## Explanation

All active timers are removed and delivered immediately with the new time, regardless of their former expiration times. The last-check time is then reset.

## Possible causes

- The wall clock moved backward relative to the previous check.

## What to check

1. Check the time change and the sessions whose timers were released.
