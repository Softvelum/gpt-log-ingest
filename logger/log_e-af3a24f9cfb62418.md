# AJA capture pixel format failed

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-af3a24f9cfb62418

## Log pattern

```text
init failed device=%s/%d reason='unsupported framebuffer format'
```

## Parameters

- Argument 1 — Device: Configured AJA device selector or display name, as indicated by the message.
- Argument 2 — Channel: Numeric AJA channel identifier used by the device API; it is not converted to a one-based display number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
init failed device=0/0 reason='unsupported framebuffer format'
```

## Short description

AJA capture pixel format failed.

## Trigger condition

Receiver initialization fails because the card rejects the capture pixel format.

## Explanation

Input-source and video setup are not attempted.

## Possible causes

- Receiver initialization fails because the card rejects the capture pixel format.

## What to check

1. Check the preceding device or capture pixel format error details.

## Related log messages

- [AJA capture frame-buffer format is unsupported](log_e-5a7863c70b616adb.md)
