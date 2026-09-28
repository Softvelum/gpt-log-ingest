# Quick Sync encoder hardware display unavailable

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_d
Log level: debug
Documentation ID: log_d-5ae6f5301cf989cd

## Log pattern

```text
%p no hw found
```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 no hw found
```

## Short description

Quick Sync encoder hardware display unavailable.

## Trigger condition

On Linux, opening and initializing a VA display fails after a hardware encoder implementation was selected.

## Explanation

Encoder session setup returns failure.

## Possible causes

- No suitable accessible render device or usable VA display could be initialized.

## What to check

1. Check preceding VA device diagnostics, render-device access, and the video driver.

## Recommended action

Restore access to a compatible device and restart encoding.
