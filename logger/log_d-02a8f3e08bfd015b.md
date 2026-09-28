# RTMP playback substitutes the last key frame

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-02a8f3e08bfd015b

## Log pattern

```text
s=%d, send last key frame %lld
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.
- Argument 2 — Timestamp: Intended replacement RTMP timestamp in milliseconds; its 32-bit value does not match the 64-bit formatting conversion.

## Log examples

No reliable rendered example is available because the timestamp argument is 32-bit while the log conversion expects a 64-bit value.

## Short description

RTMP playback substitutes the last key frame.

## Trigger condition

After advertising, the first selected live inter-frame is replaced with the saved last key frame when key-frame substitution is enabled.

## Explanation

The replacement uses the selected live item timestamp so playback can resume with a key frame. The printed timestamp is unreliable because a 32-bit value is supplied to a 64-bit conversion.

## Possible causes

- After advertising, the first selected live inter-frame is replaced with the saved last key frame when key-frame substitution is enabled.

## What to check

1. Check the post-advertising transition and whether a new live key frame has arrived.

## Recommended action

No action is required for an expected recovery transition.
