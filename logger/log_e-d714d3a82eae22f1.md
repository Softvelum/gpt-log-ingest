# NVIDIA context remains in use during release

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_e
Log level: error
Documentation ID: log_e-d714d3a82eae22f1

## Log pattern

```text
nvenc ctx=%p is alreary in use
```

## Parameters

- Argument 1 — Context: CUDA context handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
nvenc ctx=0x12340000 is alreary in use
```

## Short description

NVIDIA context remains in use during release.

## Trigger condition

Manager release finds a context still recorded in the in-use context map.

## Explanation

The context is nevertheless passed to CUDA destruction and removed from tracking.

## Possible causes

- A checked-out context has not been returned before global manager release.

## What to check

1. Check transcoder worker shutdown and context-return ordering for this handle.

## Recommended action

Ensure users of NVIDIA contexts stop and release them before manager teardown.

## Related log messages

- [NVIDIA context destruction was issued](log_i-667c46409358a841.md)
