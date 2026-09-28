# Bus-signal recovery handler could not be installed

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-fdbb830098c4ab1d

## Log pattern

```text
failed to set SIGBUS handler errno=%d
```

## Parameters

- Argument 1 — System error: Integer operating-system error number after handler installation failed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set SIGBUS handler errno=22
```

## Short description

Bus-signal recovery handler could not be installed.

## Trigger condition

Installing the non-Windows SIGBUS handler fails.

## Explanation

The installation helper returns false. The transmuxer worker ignores that result and continues its run loop, so the expected handler may be unavailable.

## Possible causes

- Installing the non-Windows SIGBUS handler fails.

## What to check

1. Check the operating-system error and signal setup; expected mapped-file recovery is unavailable if installation fails.

## Recommended action

Check the operating-system error and signal setup; expected mapped-file recovery is unavailable if installation fails.
