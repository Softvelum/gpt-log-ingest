# AJA capture video setup failed

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-c3171a67c1df7dcd

## Log pattern

```text
init failed device=%s/%d reason='video setup failed'
```

## Parameters

- Argument 1 — Device: Configured AJA device selector or display name, as indicated by the message.
- Argument 2 — Channel: Numeric AJA channel identifier used by the device API; it is not converted to a one-based display number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
init failed device=0/0 reason='video setup failed'
```

## Short description

AJA capture video setup failed.

## Trigger condition

Receiver initialization fails because device video setup fails.

## Explanation

Capture initialization stops before audio setup and host buffers.

## Possible causes

- Receiver initialization fails because device video setup fails.

## What to check

1. Check the preceding device or capture video setup error details.

## Related log messages

- [AJA capture has no recognized input video format](log_e-6e21743a1b8a800f.md)
