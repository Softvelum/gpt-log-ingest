# AJA reader thread creation raised an exception

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-a3b114fecc0247ca

## Log pattern

```text
failed to start AJA reader worker id=%llu name='%s': %s
```

## Parameters

- Argument 1 — Session ID: Positive unsigned 64-bit identifier for the reader or writer session.
- Argument 2 — Session name: Configured AJA device and channel name.
- Argument 3 — Exception detail: Text returned by the thread-creation exception.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to start AJA reader worker id=1 name='0/0': thread creation failed
```

## Short description

AJA reader thread creation raised an exception.

## Trigger condition

Creating the reader worker thread throws a standard exception.

## Explanation

The worker is marked finished and startup returns failure.

## Possible causes

- Creating the reader worker thread throws a standard exception.

## What to check

1. Check the exception detail associated with the session.

## Related log messages

- [AJA reader worker did not start](log_e-8f2084a1332a52c1.md)
