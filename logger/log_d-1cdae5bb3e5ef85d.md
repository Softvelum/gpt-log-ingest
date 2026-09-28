# SDI device output is ready

Product: Nimble Streamer
Component: SDI device output
Logging function: log_d
Log level: debug
Documentation ID: log_d-1cdae5bb3e5ef85d

## Log pattern

```text
sdi output(%s) created [%s/%s]
```

## Parameters

- Argument 1 — Media: Output media selection, text: va means video and audio; v means video only.
- Argument 2 — Device type: FFmpeg output device type, text.
- Argument 3 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
sdi output(va) created [decklink/live/camera]
```

## Short description

SDI device output is ready.

## Trigger condition

Device initialization, including header writing when required, has succeeded.

## Explanation

The output is marked created and can accept media.
