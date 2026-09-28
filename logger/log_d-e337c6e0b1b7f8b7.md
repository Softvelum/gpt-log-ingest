# HLS rotation key position selected

Product: Nimble Streamer
Component: DRM key management
Logging function: log_d
Log level: debug
Documentation ID: log_d-e337c6e0b1b7f8b7

## Log pattern

```text
stream='%s/%s' duration=%llu key_pos=%d
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Stream duration: Unsigned accumulated stream duration in milliseconds.
- Argument 4 — Key position: Integer index of the selected key in the rotating key set.

## Log examples

Illustrative examples; these are not captured server logs.

```text
stream='live/camera' duration=120000 key_pos=0
```

## Short description

HLS rotation key position selected.

## Trigger condition

VCAS HLS encryption calculates the current key position from stream duration and the rotation interval.

## Explanation

The selected key is checked for availability. A zero rotation interval always selects position zero.

## Possible causes

- VCAS HLS encryption calculates the current key position from stream duration and the rotation interval.
