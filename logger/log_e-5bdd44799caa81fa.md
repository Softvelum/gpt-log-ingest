# Cannot make SRT socket nonblocking

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-5bdd44799caa81fa

## Log pattern

```text
set non blocking failed, s=%d
```

## Parameters

- Argument 1 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
set non blocking failed, s=100
```

## Short description

Cannot make SRT socket nonblocking.

## Trigger condition

Configuring a newly created socket or group for nonblocking sends and receives fails.

## Explanation

The new socket or group is closed and creation reports failure.

## Possible causes

- Configuring a newly created socket or group for nonblocking sends and receives fails.

## What to check

1. Inspect preceding SRT option errors.

## Related log messages

- [Cannot configure SRT blocking behavior](log_e-5ae908264ac99a09.md)
