# Media subprocess log level changed

Product: Nimble Streamer
Component: Media subprocess communication
Logging function: log_i
Log level: info
Documentation ID: log_i-1650ffe40a010bbe

## Log pattern

```text
log level changed to '%s'
```

## Parameters

- Argument 1 — Requested level: Exact log-level text received from the parent process.

## Log examples

Illustrative examples; these are not captured server logs.

```text
log level changed to 'debug'
```

## Short description

Media subprocess log level changed.

## Trigger condition

A log-mode control message is received and its text is converted to a logging level.

## Explanation

The new level is applied before this informational message is attempted. A level that filters informational messages can therefore suppress the confirmation.

## Possible causes

- A parent-process command changes logging verbosity.

## What to check

1. Check whether the new level permits informational logging.
